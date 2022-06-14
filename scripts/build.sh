docker run --rm -v "$PWD":/app -w /app rustbuildserver cargo build
# docker run --rm -v "$PWD":/app -w /app rustbuildserver cargo build --release