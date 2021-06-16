# What's this?

Just a tiny web server to host static web app.  Built in Rust.  Alpine enabled.
* 6.63 MB on docker hub.

## prepare vm 
* prepare vm or another machine running Alpine, 
* enable ssh connection, 
* edit the configurations in build.sh so that the build process can build the binary on Alpine and rsync back the binary to the .build folder

## build the wwwloader
```sh
./build.sh
```

## edit start.sh
* un-remark the exports below for local test by using docker_run.sh
* remark it if you want to push to dockerhub, because the variable can be configured at docker-compose or kubernetes deployment. no need hardcode inside the docker.
```sh
export SERVICE_NAME="tiny-in-alpine"
export WWW_FOLDER="www"
export LISTEN_ON="0.0.0.0:3030"
```

## test build and run 
```sh
docker_run.sh
```

## test a completed example by pulling from dockerhub
* check example folder.
```sh
cd example
docker-compose up --build
```

## Troubleshooting
* error when running ./docker_run.sh
```sh
thread 'main' panicked at 'Environment variable SERVICE_NAME is not defined: NotPresent', src/config.rs:17:27
```
* solution: read the "edit start.sh" above.