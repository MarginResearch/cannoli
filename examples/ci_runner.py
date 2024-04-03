import os
import subprocess
import time

# get configuration
server = os.environ["CANNOLI_SERVER"]
qemu   = os.environ["CANNOLI_QEMU"]
jitter = os.environ["CANNOLI_JITTER"]
app    = os.environ["CANNOLI_TARGET"]

# start cannoli server
print(f"executing server {server}")
server = subprocess.Popen([server])

# give the server time to bind to a local tcp port
time.sleep(0.5)

# run example app through qemu cannoli (to completion)
print(f"running {qemu} -L . -cannoli {jitter} {app}")
subprocess.run([qemu, "-L", ".", "-cannoli", jitter, app], check=True, timeout=5)

# wait for server to finish
try:
    print("qemu done, waiting for server")
    ret = server.wait(10)
    assert ret == 0
except subprocess.TimeoutExpired:
    # timeouts are ok
    server.kill()
