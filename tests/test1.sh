#!/bin/bash


cd ${BASH_SOURCE%/*}

for i in {1..5}
do
   cargo run < test
done