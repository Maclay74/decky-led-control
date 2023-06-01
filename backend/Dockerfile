# Use a minimal base image with .NET 7 SDK
FROM mcr.microsoft.com/dotnet/sdk:7.0.302-bullseye-slim-amd64 AS build

# Install make
RUN apt-get update && \
    apt-get install -y make build-essential libc6-dev

# Run building script
ENTRYPOINT [ "/backend/entrypoint.sh" ]