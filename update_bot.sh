#!/bin/bash

export PATH=$PATH:/root/.cargo/bin

if ! type -P cargo &> /dev/null; then
  echo "cargo is not installed or not accessible"
  exit 1
fi

cd /root/AutoUpdateTestA

git fetch origin
HEADHASH="$(git rev-parse HEAD)"
UPSTREAMHASH="$(git rev-parse @{u})"
echo "Checking Update"
if [ "$HEADHASH" != "$UPSTREAMHASH" ]; then
        pkill -f "AutoUpdateTestA"
        git pull
        "Running"
        cargo run &
fi