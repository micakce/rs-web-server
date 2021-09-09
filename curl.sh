#!/bin/bash

curl localhost:7878/sleep && echo "Done 1" &
curl localhost:7878/ && echo "Done 2" &
curl localhost:7878/sleep && echo "Done 3" &
curl localhost:7878/ && echo "Done 4" &
curl localhost:7878/sleep && echo "Done 5" &
curl localhost:7878/sleep && echo "Done 6" &
curl localhost:7878/idaho && echo "Done 7" &

wait
