#!bin/sh

seq 1 200 | xargs -I $ -n1 -P10  curl --request POST \
  --url http://localhost:7878/ \
  --header 'Content-Type: application/json' \
  --data '{"message": "hello world"}'