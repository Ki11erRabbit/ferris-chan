#!/bin/sh

curl -X GET localhost:3000/post -d '{"board": "Technology, "category": "Interests", "start": 0, "count": 1 }' -H 'Content-Type: application/json'