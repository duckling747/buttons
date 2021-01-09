# Buttons
The greatest buttons online. 
## Compilation
Ensure that you have Rust and Cargo installed. Run `cargo build` to build the development version and `cargo build 
--release` to build the production one. You can run the program locally using `cargo run` and `cargo run --release`, 
respectively.
## Docker
Build the docker image by issuing e.g. `docker build -t buttonsapp .`. Note that the webapp runs on port 8000, so for running the app via docker you'd use e.g. `docker run --rm -it -p 8000:8000 buttonsapp`.
## Heroku
The app can also be run on Heroku, [here](https://buttonsappwebmaster.herokuapp.com/), if you don't care to run it from your 
machine.
## Docker Hub
The app is in docker hub, if for some reason you don't want to compile it and don't want to use Heroku. Just use `docker run --rm -d -p 80:8000 poweruserdockerman/buttonsapp` and go to localhost with e.g. your browser to have that sweet excitement in your life.
