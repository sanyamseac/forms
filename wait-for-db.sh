#!/bin/bash
set -e

HOST=${SCYLLA_URI%%:*}
PORT=${SCYLLA_URI##*:}

echo "Waiting for ScyllaDB at $HOST:$PORT..."
until nc -z $HOST $PORT; do
  echo "ScyllaDB is unavailable - sleeping"
  sleep 1
done
echo "ScyllaDB is up - starting application"

exec "$@"