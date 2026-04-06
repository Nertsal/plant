list:
    just --list

web command *ARGS:
    cargo geng {{command}} --platform web --release {{ARGS}}

# Build the Demo version of the game for all platforms
build-demo:
    just build-all-platforms ./target/release-demo --features demo

docker_image := "ctl-build-docker"

build-docker:
    docker build -t {{docker_image}} .

build-web *ARGS:
    # Itch-Web
    CARGO_TARGET_DIR=./target/web \
    cargo geng build --release --platform web {{ARGS}}
    cd ./target/web/geng && zip -FS -r ../../web.zip ./*

# publish-itch:
#     CARGO_TARGET_DIR=`pwd`/target/release-demo/web cargo geng build --release --platform web --out-dir `pwd`/target/release-demo/web
#     butler -- push `pwd`/target/release-demo/web nertsal/plantile:html5
