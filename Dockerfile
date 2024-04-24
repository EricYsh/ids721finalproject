# Use a Rust image as the base builder
FROM rust:1.68 as builder

# Install Python and pip
RUN apt-get update && \
    apt-get install -y python3 python3-pip && \
    ln -s /usr/bin/python3 /usr/bin/python

# Set the working directory
WORKDIR /usr/src/app

# Copy your source code
COPY . .
# Build Rust application
RUN cargo build --release

# Install Python dependencies
RUN pip install -r requirements.txt


# Use a Debian slim image as the runtime environment
FROM debian:stable-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y python3 python3-pip && \
    ln -s /usr/bin/python3 /usr/bin/python

# Copy the built Rust binary and Python script from the builder stage
COPY --from=builder /usr/src/app/target/release/ids721-final /usr/local/bin/ids721-final
COPY --from=builder /usr/src/app/app.py /usr/local/bin/app.py
COPY --from=builder /usr/src/app/requirements.txt /usr/local/bin/requirements.txt
# Install Python dependencies
RUN pip install -r requirements.txt

# Set the working directory
WORKDIR /usr/local/bin

# Expose the port the app runs on
EXPOSE 8080

#Run the app
ENTRYPOINT ["/usr/local/bin/ids721-final"]

