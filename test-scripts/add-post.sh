#!/bin/sh

curl -X POST localhost:3000/post -d '{"board": "Technology", "category": "Interests", "image": "", "text": "CHICKEN JOCKEY" }' -H 'Content-Type: application/json'