default:
    @just --list

# Run the project locally
watch $RUST_BACKTRACE="1":
    cd ./gh-pages-app && trunk serve

# build-tw:
#     tailwindcss -c ./tailwind.config.js -i ./input.css -o ./style/output.css
release:
    cd ./gh-pages-app && trunk build --release
fmt:
    leptosfmt ./gh-pages-app/**/*.rs
    leptosfmt ./shared/**/*.rs
# watch-tw:
#     tailwindcss -c ./tailwind.config.js -i ./input.css -o ./style/output.css --watch
