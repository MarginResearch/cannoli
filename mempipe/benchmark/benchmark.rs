use mempipe::{SendPipe, RecvPipe};

/// Benchmark was run on a custom init, thus no other processes or threads
/// running on the Linux system
fn main() {
    fn pin_core(core: usize) {
        unsafe {
            let mut cpuset = [0u8; 64];
            cpuset[core / 8] |= 1 << (core % 8) as u8;
            assert!(libc::sched_setaffinity(
                0, cpuset.len(), cpuset.as_ptr() as *const _) != -1);
        }
    }

    // Create `/proc`
    std::fs::create_dir_all("/proc").unwrap();

    // Mount procfs
    unsafe {
        while libc::umount(c"/proc".as_ptr()) != -1 {}

        if libc::mount(c"proc".as_ptr(), c"/proc".as_ptr(),
                       c"proc".as_ptr(),
                       0, std::ptr::null_mut()) == -1 {
            panic!("{:?}", std::io::Error::last_os_error());
        }
    }

    std::fs::create_dir_all("/dev/shm").unwrap();

    println!("{}", std::fs::read_to_string("/proc/cpuinfo").unwrap());

    /*
    for file in std::fs::read_dir("/proc").unwrap() {
        let file = file.unwrap();
        if let Ok(pid) = file.path().file_name().unwrap().to_str().unwrap().parse::<i32>() {
            println!("{}", std::fs::read_to_string(format!("/proc/{pid}/stat")).unwrap());
        }
    }*/

    for core in 0..192 {
        for core2 in 0..192 {
            pin_core(core);

            let iters = if core == core2 {
                100
            } else {
                1000000
            };

            // We should fail to make a pipe with a mismatched size
            let mut tx = SendPipe::<1, 4>::create().unwrap();
            let id = tx.uid();

            let thr = std::thread::spawn(move || {
                pin_core(core2);

                let rx = RecvPipe::<1, 4>::open(id).unwrap();
                let mut ticket = rx.request_ticket();
                let mut foop = 0;
                while foop < iters {
                    let (new_ticket, res) = rx.try_recv(ticket, |_| -> Result<(), ()> {
                        Ok(())
                    });

                    if res.is_some() {
                        foop += 1;
                    }

                    ticket = new_ticket;
                }
            });

            let it = rdtsc();
            for _ in 0..iters {
                tx.alloc_buffer(false);
            }

            let _ = thr.join().unwrap();
            let elapsed = rdtsc() - it;

            println!("{core:3} {core2:3} {:14.3}",
                elapsed as f64 / iters as f64);
        }
    }
}

