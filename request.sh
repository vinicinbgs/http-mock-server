#!bin/sh

seq 1 1 | xargs -I $ -n1 -P10  curl --request POST \
  --url http://localhost:7878/register/10 \
  --header 'Content-Type: application/json'