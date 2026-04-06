#!/bin/bash

echo "Hitting backend with dummy data"

curl -X POST http://localhost:3000/record \
    -H "Content-Type: application/json" \
    -d '{
        "id": "user_99",
        "email": "hello@example.com"
        }'