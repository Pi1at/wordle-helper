default:
    @just --list

# Run the project locally
watch $RUST_BACKTRACE="1":
    trunk serve --port 4242

# build-tw:
#     tailwindcss -c ./tailwind.config.js -i ./input.css -o ./style/output.css
release:
    trunk build --release
fmt:
    leptosfmt ./**/*.rs
# watch-tw:
#     tailwindcss -c ./tailwind.config.js -i ./input.css -o ./style/output.css --watch
