#!/bin/sh

curl -X PUT localhost:3000/auth/register -d '{"username": "test", "email": "test@test.com", "password": "test" }' -H 'Content-Type: application/json'