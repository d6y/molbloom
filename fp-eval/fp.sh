#!/bin/bash
set -eu
set -o pipefail

# Evaluates the false positive rate of the Bloom filter against different sizes of filter.

# To do the evaluation, SureChEMBL data split into two: 
# the first half used to build the filter; the second used to check for false positives (none expected to be found).
#
# Outpuut: for each size of model, the model iteself and a file containing the count of
# false positives found from the test file.

# Useful if you want to debug this script by viewing each line being exectured:
set -x

# See below for the source of these files.
TRAIN_FILE=smiles-split-00
TEST_FILE=smiles-split-01

# How many items do we expect to store?
n=$(wc -l < "$TRAIN_FILE")

# Where models and results are saved:
OUT_DIR="out"
mkdir -p $OUT_DIR

# A CSV file we append to with bits, fp_count, fp_rate
# We create it aftresh each run
CSV="$OUT_DIR/fp.csv"
rm -f $CSV

# Run: `cargo build --release` to get this binary:
APP=../target/release/molbloom

  # M is the number of bits in the filter
for M in 10 100 1000 10000 50000 100000 500000 1000000 2000000 3000000 4000000 5000000 10000000 50000000 60000000 70000000 80000000 100000000 150000000 250000000 500000000 1000000000 5000000000 10000000000 
do
  filter_file="$OUT_DIR/fp-test-${M}.bin"
  count_file="$OUT_DIR/fp-count-${M}.txt"

  if [ -f $filter_file ]; then
    echo "Filter ${filter_file} exists, no need to create"
  else
    echo "Building ${filter_file} with filter size ${M} bits"
    $APP -f ${filter_file} build --num-bits $M --num-items $n < $TRAIN_FILE
  fi

  if [ -f $count_file ]; then
    echo "$count_file exists, skipping"
  else
    echo "Evaluating ${filter_file} for false positives: count of FP in $count_file"
    $APP -f ${filter_file} query < $TEST_FILE | grep true | wc -l > ${count_file}
  fi

   fp_count=$(cat $count_file)
   fp_rate=$(echo "scale=10; $fp_count / $n" | bc) # assumes n is the same as TEST_FILE length
   echo "$M,$fp_count,$fp_rate" >> $CSV

 done

# Where do those data files come from?
#
# Source is the SureChEMBL map files:
# SureChEMBL_map_20141231.txt.gz to SureChEMBL_map_20240101.txt.gz
#
# Extracted SMILES:
# gzip -cd *.txt.gz | cut -d$'\t' -f 2 > smiles.txt
#
# Count:
# wc -l secret/surechembl/map/smiles.txt
# 373616491 secret/surechembl/map/smiles.txt
#
# Remove duplicates:
# sort secret/surechembl/map/smiles.txt | uniq > smiles-sorted-unique.txt
# wc -l smiles-sorted-unique.txt
# 23465171 smiles-sorted-unique.txt
#
# Sorted SMILES not a problem for the Bloom filter, but could be a problem
# for false positive testing, so we shuffle the unique SMILES
# brew install coreutils
# shuf smiles-sorted-unique.txt > smiles-unique-shuffled.txt
# 
# Split into a training set and a testing set:
# split -l 11732585 -d smiles-unique-shuffled.txt smiles-split-
# wc -l smiles-split-0*
#  11732585 smiles-split-00
#  11732585 smiles-split-01
#        1 smiles-split-02
#  23465171 total
# 
# Ok, we lost one but we can live with that.
