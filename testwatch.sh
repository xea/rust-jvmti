#!/bin/bash

while true; do
	inotifywait -r --exclude ".*swp" -e modify src tests
	cargo test
done
