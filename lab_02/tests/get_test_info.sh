#!/bin/bash

TARGET=test.exe

./$TARGET > /dev/null
result=$?

num_success=$(( result & 0xFFFFFFFF ))
num_failed=$(( (result >> 32) & 0xFFFFFFFF ))

TESTED_FUNC="out/matrix_mult.gcno"
REPORT="tests/stud-unit-test-report-prev.json"

coverage=$(gcov $TESTED_FUNC | grep "Lines executed" | cut -d':' -f2 | cut -d'%' -f1)
coverage=$(echo "scale=2; $coverage / 100" | bc)
if [[ "$coverage" == .* ]]; then
    coverage="0$coverage"
fi

JSON_DATA="{
    \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%S%z")\",
    \"coverage\": $coverage,
    \"passed\": $num_success,
    \"failed\": $num_failed
}"

echo "$JSON_DATA" > "$REPORT"