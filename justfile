set dotenv-required := true
set dotenv-filename := ".env"

node_arch := if arch() == "aarch64" { "arm64" } else { "x64" }

# List all available commands
default:
    just --list

# ==============================================================================
# Onboarding
# ==============================================================================

# Install other dependencies used during development
install-system-dependencies:
    # provides cargo set-version, used by the release process
    cargo +stable install cargo-edit
    npm install

# ==============================================================================
# Write
# ==============================================================================

# Fix formatting, indentation etc of all files
format:
    cargo +nightly fmt --all -- --verbose
    cargo clippy --fix --allow-dirty

# ==============================================================================
# Lint
# ==============================================================================

# Run all linting checks
lint:
    just check-cargo
    just check-formatting
    just check-clippy
    just check-versions

# Cargo.toml and package.json must publish the same version
check-versions:
    #!/usr/bin/env bash
    set -euo pipefail

    CARGO_VERSION=$(cargo pkgid | sed 's/.*[#@]//')
    NPM_VERSION=$(node -p "require('./package.json').version")
    if [ "$CARGO_VERSION" != "$NPM_VERSION" ]; then
        echo "Version mismatch: Cargo.toml has $CARGO_VERSION but package.json has $NPM_VERSION"
        echo "Run: cargo set-version '$NPM_VERSION' (versions are bumped together by 'just create-release-commit')"
        exit 1
    fi
    echo "Versions in sync: $CARGO_VERSION"

# Run cargo check
check-cargo:
    cargo check --locked

# Check for formatting issues (nightly: rustfmt.toml uses nightly-only options)
check-formatting:
    cargo +nightly fmt --all -- --check

# Check for clippy warnings
check-clippy:
    cargo clippy --all-targets -- -D warnings

# ==============================================================================
# Test
# ==============================================================================

# Run all tests
test:
    cargo test -- --color=always

# Run all tests and generate a coverage report
coverage:
    rm -rf target/llvm-cov/html
    cargo llvm-cov test --html --ignore-run-fail --ignore-filename-regex '(_test.rs|/test.rs)'

# ==============================================================================
# Build
# ==============================================================================

# Build the npm packages for this machine's architecture and install them locally
build-local:
    #!/usr/bin/env bash
    set -euxo pipefail

    rm -rf npm/packages
    just --dotenv-filename .env.darwin-{{ node_arch }} build-binary-package
    just --dotenv-filename .env.darwin-{{ node_arch }} create-npm-root-package
    just patch-local
    cd npm/packages/imageoptim-cli
    npm install

# Modify the local package.json file to only have this machine's optionalDependency
patch-local:
    #!/usr/bin/env node
    const fs = require("fs");
    const path = require("path");
    const arch = process.arch;
    const srcPath = path.resolve("npm/packages/imageoptim-cli/package.json");
    const pkg = require(srcPath);
    const nextPkg = {
        ...pkg,
        optionalDependencies: {
            [`imageoptim-cli-darwin-${arch}`]: `file:../imageoptim-cli-darwin-${arch}`
        }
    };
    const json = JSON.stringify(nextPkg, null, 2);
    console.log(json);
    fs.writeFileSync(srcPath, json);

# Smoke test the locally built npm package
test-local:
    #!/usr/bin/env bash
    set -euxo pipefail

    node npm/packages/imageoptim-cli/index.cjs --version
    node npm/packages/imageoptim-cli/index.cjs --help > /dev/null
    echo "npm package OK"

# Build a rust binary and corresponding npm package for a specific target
build-binary-package:
    just create-rust-binary
    just create-npm-binary-package

# Build a rust binary for a specific target
create-rust-binary:
    #!/usr/bin/env bash
    set -euxo pipefail

    cargo build --release --locked --target "$TARGET"

# Once a rust binary for a specific target has been built, create an npm package for it
create-npm-binary-package:
    #!/usr/bin/env bash
    set -euxo pipefail

    rm -rf "$NODE_PKG_DIR_PATH"
    mkdir -p "$NODE_PKG_DIR_PATH/bin"
    cp "$RUST_BINARY_PATH" "$NODE_PKG_RUST_BINARY_PATH"
    cp README.md "$NODE_PKG_DIR_PATH/README.md"
    just create-npm-binary-package-json

# Create the package.json file for an npm package for a specific target
create-npm-binary-package-json:
    #!/usr/bin/env node
    const fs = require("fs");
    const path = require("path");
    const srcPath = path.resolve("package.json");
    const destPath = path.resolve(process.env.NODE_PKG_DIR_PATH, "package.json");
    const pkg = require(srcPath);
    const nextPkg = {
        ...pkg,
        bin: undefined,
        contributors: undefined,
        dependencies: undefined,
        devDependencies: undefined,
        engines: undefined,
        keywords: undefined,
        optionalDependencies: undefined,
        name: process.env.NODE_PKG_NAME,
        description: `Rust Binary for ${process.env.NODE_OS} ${process.env.NODE_ARCH}`,
        os: [process.env.NODE_OS],
        cpu: [process.env.NODE_ARCH],
    };
    const json = JSON.stringify(nextPkg, null, 2);
    console.log(json);
    fs.writeFileSync(destPath, json);

# Create the parent npm package which delegates to each target-specific package
create-npm-root-package:
    #!/usr/bin/env bash
    set -euxo pipefail

    rm -rf "$NODE_ROOT_PKG_DIR_PATH"
    mkdir -p "$NODE_ROOT_PKG_DIR_PATH"
    cp README.md "$NODE_ROOT_PKG_DIR_PATH/README.md"
    cp npm/index.cjs "$NODE_ROOT_PKG_DIR_PATH/index.cjs"
    just create-npm-root-package-json

# Create the package.json file for the parent npm package
create-npm-root-package-json:
    #!/usr/bin/env node
    const fs = require("fs");
    const path = require("path");
    const srcPath = path.resolve("package.json");
    const destPath = path.resolve(process.env.NODE_ROOT_PKG_DIR_PATH, "package.json");
    const pkg = require(srcPath);
    const nextPkg = {
        ...pkg,
        devDependencies: undefined,
        bin: {
          imageoptim: "./index.cjs",
        },
        optionalDependencies: {
          "imageoptim-cli-darwin-x64": pkg.version,
          "imageoptim-cli-darwin-arm64": pkg.version,
        },
    };
    const json = JSON.stringify(nextPkg, null, 2);
    console.log(json);
    fs.writeFileSync(destPath, json);

# ==============================================================================
# Publish
# ==============================================================================
# Create a tagged, versioned commit: bumps package.json, then cargo set-version

# keeps Cargo.toml in lock-step (see .release-it.json)
create-release-commit:
    #!/usr/bin/env bash
    set -euxo pipefail

    npm exec release-it -- --increment pre

# Publish the npm package for a specific target
publish-npm-binary-package:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd "$NODE_PKG_DIR_PATH"
    npm publish --access public ${NPM_PUBLISH_FLAGS:-}

# Publish the parent npm package
publish-npm-root-package:
    #!/usr/bin/env bash
    set -euxo pipefail

    cd "$NODE_ROOT_PKG_DIR_PATH"
    npm publish --access public ${NPM_PUBLISH_FLAGS:-}
