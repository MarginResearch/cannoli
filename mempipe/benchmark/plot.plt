set term pngcairo size 1440,900
set grid
set title "Cannoli mempipe message sending latency on a 96C/192T machine (4x Xeon 6252N)\nRaw data: https://gist.github.com/gamozolabs/3ec84e2fa698a09f96ca4301c0fad88b"
set xlabel "Sender core"
set ylabel "Receiver core"
set cblabel "Latency (in cycles, divide by 2.3e9 for seconds)"
set xrange [0:191]
set yrange [0:191]
plot "mempipe_benchmark.txt" u 1:2:3 w image

