set term wxt size 700,500 persist
set xlabel "Time (seconds)"
set ylabel "Million target instructions/second"
set grid mxtics xtics ytics
set ytics 100
set key bottom

last = 0
last_time = 0

# number of points in moving average
n = 100

array A[n]

samples(x) = $0 > (n-1) ? n : int($0+1)
mod(x) = int(x) % n
avg_n(x) = (A[mod($0)+1]=x, (sum [i=1:samples($0)] A[i]) / samples($0))

process(x) = (old_last = last, old_last_time = last_time, last = $1, last_time = $2, ($1 - old_last) / ($2 - old_last_time) / 1e6)

plot "log_1thr_256k_16.txt" u 2:(avg_n(process(x))) w l t "256k chunk, 16 #buf, 1 consumer", \
    "log_2thr_256k_16.txt" u 2:(avg_n(process(x))) w l t "256k chunk, 16 #buf, 2 consumers", \
    "log_3thr_256k_16.txt" u 2:(avg_n(process(x))) w l t "256k chunk, 16 #buf, 3 consumers", \
    "log_4thr_256k_16.txt" u 2:(avg_n(process(x))) w l t "256k chunk, 16 #buf, 4 consumers"

