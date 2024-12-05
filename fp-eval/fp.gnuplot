set datafile separator ","
set xlabel "Size (bits)"
set ylabel "False Positive Rate"

set term pngcairo size 800,600 font "Menlo,12"

# You know you want to:
# set term pngcairo size 800,600 font "Comic Sans MS,12"

# Slightly more than 0-100 so you can see the lines
set yrange [-3:103]


# LLM generated:
set logscale x 10  # Set logarithmic scale for the x-axis (base 10)
set xrange [1:1e10]  # Set range for x-axis from 1 to 10^10
set xtics format "10^{%L}"  # Display x-axis labels as powers of 10

set key at 1500,10 Left reverse

# Dataset specific settings:
set title "SureChEMBL 50/50 FPR Evaluation"
set output "surechembl_fp_rate_vs_bits.png"
n_items=11732585

plot "out/fp.csv" using 1:($3*100) with linespoints lw 2 title "Actual FPR", \
     "out/fp.csv" using 1:(100 * 0.5**(($1 / n_items) * log(2))) with lines lw 2 dashtype 2 title "Theoretical FPR"

