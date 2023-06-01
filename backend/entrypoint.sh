#!/bin/sh
set -e

echo "Container's IP address: `awk 'END{print $1}' /etc/hosts`"

# export DOTNET_ROOT=/usr/share/dotnet/
# export PATH=$PATH:$DOTNET_ROOT

# Copy the project file(s) to the container
cd /backend

# Build the application
dotnet restore --runtime linux-x64
dotnet build
dotnet publish -c Release -r linux-x64 --self-contained true /p:PublishSingleFile=true /p:PublishTrimmed=true -o /backend/bin

# Build EC tool
cd ./ectool
make clean
make