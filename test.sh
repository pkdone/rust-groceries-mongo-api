#!/bin/bash

if ! which jq > /dev/null; then
    printf "'jq' utility is not on the path - install it first - e.g.: sudo apt install jq"
fi

printf "\nInitial HTTP GET output: \n"
curl -sS --location --request GET 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'

printf "\nDelete Apples HTTP DELETE output: \n"
curl -sS --location --request DELETE 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apples"
}'

printf "\nTest no apples HTTP GET result: \n"
if ! curl -sS --location --request GET 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' | jq '.[] | select(.name=="apples")' | grep apples; then
    printf "\n----OK: No apples exist\n"
else
    printf "\n----ERROR: Apples exist\n"
    exit 1
fi

printf "\nAdd Apples HTTP POST output: \n"
curl -sS --location --request POST 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apples",
    "quantity": 3
}'

printf "\nTest apples exist HTTP GET result: \n"
if ! curl -sS --location --request GET 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' | jq '.[] | select(.name=="apples")' | grep apples; then
    printf "\n----ERROR: No apples exist\n"
    exit 1
else
    printf "\n----OK: Apples exist\n"
fi

printf "\nDelete Apples HTTP DELETE output: \n"
curl --location --request DELETE 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apples"
}'

printf "\nTest no apples HTTP GET result: \n"
if ! curl -sS --location --request GET 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' | jq '.[] | select(.name=="apples")' | grep apples; then
    printf "\n----OK: No apples exist\n"
else
    printf "\n----ERROR: Apples exist\n"
    exit 1
fi

printf "\nAdd Apples HTTP PUT output: \n"
curl -sS --location --request PUT 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apples",
    "quantity": 5
}'

printf "\nTest apples exist HTTP GET result: \n"
if ! curl -sS --location --request GET 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' | jq '.[] | select(.name=="apples")' | grep apples; then
    printf "\n----ERROR: No apples exist\n"
    exit 1
else
    printf "\n----OK: Apples exist\n"
fi

printf "\n"
