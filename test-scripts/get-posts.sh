#!/bin/sh

curl -X GET localhost:3000/post -d '{"board": "Technology", "category": "Interests", "count": 1, "offset": 0 }' -H 'Content-Type: application/json'