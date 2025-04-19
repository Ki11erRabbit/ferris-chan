#!/bin/sh

curl -X POST localhost:3000/post/reply -d '{"board": "Technology", "category": "Interests", "image": "", "text": "What is happening", "parent": 1 }' -H 'Content-Type: application/json'