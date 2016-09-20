#!/bin/bash

find ./test-data -name "*.class" | while read CLASSFILE; do
    OUTFILE="${CLASSFILE}.class"
    ./target/debug/jvmti write $CLASSFILE > $OUTFILE
    HASHES=`md5 -q $CLASSFILE $OUTFILE | paste -s -d " " -`
    read -r -a RESULT <<< $HASHES

    if [ "${RESULT[0]}" != "${RESULT[1]}" ]; then
        echo " ---------------------- Mismatch found: ${RESULT[0]} ${RESULT[1]} ${CLASSFILE} -----------------------"
        javap -v $CLASSFILE > ./tmp-compare-1
        javap -v $OUTFILE > ./tmp-compare-2
        diff -u ./tmp-compare-1 ./tmp-compare-2
    fi

    rm -f $OUTFILE ./tmp-compare-1 ./tmp-compare-2
done
