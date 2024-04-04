{
  description = "Cannoli development shell";

  inputs = {
    # Use the rolling-release cycle of nixpkgs for most tools
    # flake.lock will pin this to a fixed version, update with "nix flake update"
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    # Pull in nixpkgs at a point in time where qemu-8.1.2 existed
    # Use this to find the applicable git hash:
    # https://lazamar.co.uk/nix-versions/?channel=nixpkgs-unstable&package=qemu
    qemu-nixpkgs.url = "github:NixOS/nixpkgs/6eed4c2552c41690535d08a2e071bca005226a4a";

    # Rust toolchain
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    # Builds rust crates, runs tests
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    qemu-nixpkgs,
    rust-overlay,
    crane,
    ...
  }: let
      # see qemu's configure script for possible targets:
      #   ./configure --help
      cpuTargets = [
        "aarch64"
        "aarch64_be"
        "alpha"
        "arm"
        "armeb"
        "cris"
        "hppa"
        "i386"
        "loongarch64"
        "m68k"
        "microblaze"
        "microblazeel"
        "mips"
        "mips64"
        "mips64el"
        "mipsel"
        "mipsn32"
        "mipsn32el"
        "nios2"
        "or1k"
        "ppc"
        "ppc64"
        "ppc64le"
        "riscv32"
        "riscv64"
        "s390x"
        "sh4"
        "sh4eb"
        "sparc"
        "sparc32plus"
        "sparc64"
        "x86_64"
        "xtensa"
        "xtensaeb"
      ];

      # concat "-linux-user" to the cpu names to create the actual configure strings
      hostCpuTargets = map (arch: arch + "-linux-user") cpuTargets;

      # stripped down cannoli sources that qemu needs to build
      # (prevents unnecessary rebuilds of qemu after unrelated changes in this repo)
      fs = lib.fileset;
      cannoli-headers = fs.toSource {
        root = ./.;
        fileset = fs.intersection
          (fs.gitTracked ./.)
          (fs.fileFilter ({name,...}: name == "cannoli.h") ./.);
      };

      qemuSrc = pkgs: pkgs.fetchurl {
        url = "https://download.qemu.org/qemu-8.1.2.tar.xz";
        sha256 = "sha256-VBUmp2RXbrSU0v9exGrrJT5i6ikDXRwjwKivTmzU8Ic=";
      };

      # nixpkgs overlay defining the qemu-cannoli package
      qemu-cannoli-overlay = final: prev: {
        # a function that takes a package, and applies optimizations
        optimize = pkg:
          (pkg.overrideAttrs (old: {
            # inject these flags when the compiler gets invoked
            # unfortunately won't show up in configure script outputs :(
            NIX_CFLAGS_COMPILE = [ (old.NIX_CFLAGS_COMPILE or "") ] ++ [
              "-O3"              # more optimizations than the nixpkgs default of -O2
              "-march=x86-64-v3" # this targets modern x86_64 but not including
                                 # avx512, should be a good balance of modern
                                 # features (e.g. avx2) but still run on CPUs
                                 # within the past 8 years at least.
            ];
          })).override rec {
            # use the newest gcc available, assuming it optimizes better
            stdenv = final.gcc12Stdenv;
            buildPackages.stdenv = stdenv;
          };

        qemu-cannoli = final.optimize ((final.qemu.overrideAttrs (old: {
          pname = "qemu-cannoli";
          src = qemuSrc final;
          configureFlags = old.configureFlags ++ [
            # speed up builds with these flags
            "--disable-docs"
            "--disable-tools"
            "--disable-download"
            # add the cannoli configure flag
            "--with-cannoli=${cannoli-headers}"
          ];
          patches = [ ./qemu_patches.patch ];
        })).override {
          inherit hostCpuTargets;
        });
      };

      qemu-pkgs = import qemu-nixpkgs {
        system = "x86_64-linux";
        overlays = [
          qemu-cannoli-overlay
        ];
      };

      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [
          (import rust-overlay)
        ];
      };
      lib = pkgs.lib;

      rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      # environment variable for bindgen
      LIBCLANG_PATH = "${pkgs.llvmPackages_15.libclang.lib}/lib";

      # compile dependencies once
      vendoredCrates = craneLib.vendorCargoDeps {
        cargoLock = ./Cargo.lock;
      };

      srcForCrate = cratePath: fs.toSource {
        root = ./.;
        fileset = fs.unions [
          ./Cargo.lock ./.cargo
          # common crates needed by everything in the workspace
          ./jitter ./mempipe ./cannoli ./extensions/arch
          # keep all Cargo.tomls as scaffolding for workspace
          (fs.fileFilter (file: file.name == "Cargo.toml") ./.)
          # add the source of the crate we want to compile
          (./. + ("/" + cratePath))
        ];
      };

      crateNameFromPath = cratePath: (lib.importTOML (./. + ("/" + cratePath + "/Cargo.toml"))).package.name;

      workspaceDeps = cratePath: craneLib.buildDepsOnly {
        src = ./.;
        pname = crateNameFromPath cratePath;
        cargoVendorDir = vendoredCrates;
        cargoExtraArgs = "-p ${crateNameFromPath cratePath}";
      };

      buildWorkspaceCrate = cratePath: craneLib.buildPackage {
        src = srcForCrate cratePath;
        pname = crateNameFromPath cratePath;
        # create a src/lib.rs next to every Cargo.toml
        postUnpack = "${lib.getExe pkgs.fd} Cargo.toml -tf -x mkdir -p {//}/src ';' -x touch {//}/src/lib.rs";
        cargoArtifacts = workspaceDeps cratePath;
        cargoVendorDir = vendoredCrates;
        cargoExtraArgs = "-p ${crateNameFromPath cratePath}";
        doNotLinkInheritedArtifacts = true;
        inherit LIBCLANG_PATH;
      };

      workspaceCratePaths = (lib.importTOML ./Cargo.toml).workspace.members;

      # attrSet of crateName -> derivation for this workspace
      workspaceMembers = lib.listToAttrs (lib.forEach workspaceCratePaths (path: {
        name = crateNameFromPath path;
        value = buildWorkspaceCrate path;
      }));

      # builds every crate in the workspace
      workspace = pkgs.symlinkJoin {
        name = "cannoli-workspace";
        paths = lib.attrValues workspaceMembers;
      };
    in

    with pkgs; {
      # run "nix build" to build qemu with cannoli patches. binaries will be
      # placed in ./result/bin/
      packages.x86_64-linux = rec {
        cannoli = qemu-pkgs.qemu-cannoli;

        default = runCommand "qemu-cannoli" {} ''
          mkdir -p $out/bin
          cp -vP ${cannoli}/bin/*   $out/bin/
          rm $out/bin/{qemu-ga,qemu-kvm}
        '';

        # build all workspace crates
        inherit workspace;
      };

      # ci checks, run with "nix flake check"
      checks.x86_64-linux = {
        inherit (qemu-pkgs) qemu-cannoli;

        # build and run cargo tests for all workspace crates
        inherit workspace;
      } //

      # run end-to-end examples (integration tests)
      lib.mapAttrs (name: env: pkgs.runCommand name (env // {
        # injected/derived environment variables
        CANNOLI_SERVER = "${workspaceMembers."${env.example}"}/bin/${env.example}";
        CANNOLI_JITTER = "${workspaceMembers."${env.example}"}/lib/lib${env.example}.so";
        CANNOLI_QEMU = "${qemu-pkgs.qemu-cannoli}/bin/${env.qemu}";
      }) ''
        $preRun
        ${pkgs.python3}/bin/python3 ${./examples/ci_runner.py}
        touch $out
      '') {
        symbolizer-mipsel-run = {
          example = "symbolizer";
          qemu = "qemu-mipsel";
          CANNOLI_TARGET = "${./examples/symbolizer/example_app}";
          preRun = ''cp ${./examples/symbolizer/symbols.txt} symbols.txt'';
        };
        symbolizer-riscv64-run = {
          example = "symbolizer";
          qemu = "qemu-riscv64";
          CANNOLI_TARGET = "${./examples/symbolizer/example_app64}";
          preRun = ''cp ${./examples/symbolizer/symbols.txt} symbols.txt'';
        };
      };

      # run "nix develop" to drop into a shell that has qemu with cannoli,
      # and a known-good nightly rust toolchain
      devShells.x86_64-linux.default = mkShell {
        # packages available in the shell
        buildInputs = [
          qemu-pkgs.qemu-cannoli
          gnumake
          pkgsCross.mipsel-linux-gnu.pkgsStatic.stdenv.cc
          rustToolchain
          nix
        ];

        shellHook = ''
          export LIBCLANG_PATH=${LIBCLANG_PATH}
          echo cannoli developer shell activated
        '';
      };
    };
}
