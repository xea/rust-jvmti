#!/bin/bash

BASEDIR=./test-data

if [ ! -z $1 ]; then
    BASEDIR=$1
fi

function clean_test_data {
    find $BASEDIR -name "*.out.class" -exec rm -f {} \;
}

clean_test_data

find ./test-data -name "*.class" | while read CLASSFILE; do
    echo "Checking ${CLASSFILE}"
    OUTFILE="${CLASSFILE}.out.class"
    RESULTS="regression-results"
    ./target/debug/jvmti write $CLASSFILE 
    HASHES=`md5 -q $CLASSFILE $OUTFILE | paste -s -d " " -`
    read -r -a RESULT <<< $HASHES

    if [ "${RESULT[0]}" != "${RESULT[1]}" ]; then
        echo " ---------------------- Mismatch found: ${RESULT[0]} ${RESULT[1]} ${CLASSFILE} -----------------------"
        javap -v $CLASSFILE > ./tmp-compare-1
        javap -v $OUTFILE > ./tmp-compare-2
        diff -u ./tmp-compare-1 ./tmp-compare-2 >> $RESULTS
    fi

    rm -f $OUTFILE ./tmp-compare-1 ./tmp-compare-2
done
