#!/bin/sh

curl -X GET localhost:3000/post/reply -d '{"count": 1, "offset": 0, "parent": 1 }' -H 'Content-Type: application/json'