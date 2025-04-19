#!/bin/sh

curl -X GET localhost:3000/post -d '{"start": 0, "count": 1 }' -H 'Content-Type: application/json'