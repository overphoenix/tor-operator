set shell := ["bash", "-uc"]
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

GIT_COMMIT := `git rev-parse --verify --short HEAD`

# help
help:
  @just --list

# build
build:
  @cargo build --release

# cli controller run
cli-controller-run:
  @cargo run -- controller run

# cli crd generate
cli-crd-generate:
  @cargo run -- crd generate --output ./helm/templates/crd.yaml

# docker build
docker-build:
  docker build \
    --tag agabani/tor-operator:{{GIT_COMMIT}} \
    .

# docker buildx build
docker-buildx-build:
  docker buildx build \
    --platform linux/amd64,linux/arm64 \
    --tag agabani/tor-operator:{{GIT_COMMIT}} \
    .

# docker build
docker-run: docker-build
  @docker run --rm agabani/tor-operator:{{GIT_COMMIT}}

# kube clean
kube-clean:
  @tilt down

# kube run
kube-run:
  @tilt up

# lint
lint:
  @cargo clippy
