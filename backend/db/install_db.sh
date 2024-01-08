#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Usage: $0 <database_user>"
  exit 1
fi

for file in *.sql; do
  psql -U $1 -d cv -a -f "$file"
done