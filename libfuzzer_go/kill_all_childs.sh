#!/bin/sh
kill $(ps aux | grep libfuzzer_go | grep -v grep | awk '{print $2}')