# Buttons
The greatest buttons online. 
## Compilation
Ensure that you have Rust and Cargo installed. Run `cargo build` to build the development version and `cargo build 
--release` to build the production one. You can run the program locally using `cargo run` and `cargo run --release`, 
respectively.
## Docker
Build the docker image by issuing e.g. `docker build -t buttonsapp .`. Note that the webapp runs on port 8000, so for running the app via docker you'd use e.g. `docker run --rm -it -p 8000:8000 buttonsapp`.
