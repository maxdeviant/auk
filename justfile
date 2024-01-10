# Run the build.
build:
    cargo build

# Run the tests.
test:
    cargo test

# Review `insta` snapshots.
review-snapshots:
    cargo insta test --review

# Remove obsolete `insta` snapshots.
remove-obsolete-snapshots:
    cargo insta test --unreferenced delete
