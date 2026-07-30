# -*- mode: just -*-


#!/usr/bin/env sh

# set -xe
# trap 'exit 1' INT
#
# go generate  # runs writetlds for json and go
# go build -o ./tldinfo ./cmd/tldinfo/main.go


# Define project directory
project_dir := justfile_dir()
output_dir := project_dir / "bin"

# Define binary name using current directory name
bin_name := "tldinfo"
target_bin := output_dir / bin_name
cmd_src := project_dir / "cmd" / bin_name / "main.go"

# Define Go
go := "go"
goos := env_var_or_default("GOOS", `go env GOARCH`)
goarch := env_var_or_default("GOARCH", `go env GOARCH`)
go_test := go + " test " + env_var_or_default("GO_FLAGS", "-race")
go_build := go + " build " + env_var_or_default("GO_FLAGS", "")

# Generate data
gen:
    go generate

# Build the project and write the resulting binary to the "bin/" directory
build: build-core
    mkdir -p {{output_dir}}
    cp -f {{target_bin}} {{project_dir}}

# Core build command
[private]
build-core:
    {{go_build}} -o {{target_bin}} {{cmd_src}}

# Run `go fmt` and `go vet`
fmt: vet
    # TODO: does this work?
    go fmt ./...

[private]
vet:
    go vet ./...

# Update project dependencies
update:
    # TODO: does this work?
    go get -u ./...
    go mod tidy

# Run tests
test:
    # TODO: does this work?
    # go test ./...
    # go test --all
    {{go_test}} ./...

# Generate documentation
doc: get-godoc
    # TODO: does this work?
    # go doc ./...
    godoc -http=:6060

[private, unix]
get-godoc:
    command -v godoc || go install golang.org/x/tools/cmd/godoc@latest
