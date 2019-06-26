# Timestamping: Example Service

This project demonstrates how to create simple timestamping service
using [Exonum blockchain](https://github.com/exonum/exonum).

![Timestamping demo](Screenshot.png)

## Getting started

Be sure you installed necessary packages:

* [git](https://git-scm.com/downloads)
* [Rust](https://rustup.rs/)
* [Node.js & npm](https://nodejs.org/en/download/)

## Install and run

### Using docker

Simply run the following command to start the timestamping service on 4 nodes
on the local machine:

```bash
docker run -p 8000-8008:8000-8008 exonumhub/exonum-timestamping:demo
```

Ready! Find demo at [http://127.0.0.1:8008](http://127.0.0.1:8008).

Docker will automatically pull image from the repository and
run 4 nodes with public endpoints at `127.0.0.1:8000`, ..., `127.0.0.1:8003`
and private ones at `127.0.0.1:8004`, ..., `127.0.0.1:8007`.

To stop docker container, use `docker stop <container id>` command.

### Manually

#### Getting started

Be sure you installed necessary packages:

* [git](https://git-scm.com/downloads)
* [Node.js with npm](https://nodejs.org/en/download/)
* [Rust compiler](https://rustup.rs/)

#### Install and run

Below you will find a step-by-step guide to start the service
on 4 nodes on the local machine.

Clone the project and install Rust dependencies:

```sh
git clone https://github.com/exonum/exonum

cd exonum/examples/timestamping/backend

cargo install --path .
```

Generate blockchain configuration:

```sh
mkdir example

exonum-timestamping generate-template example/common.toml --validators-count 4
```

Generate templates of nodes configurations:

<!-- markdownlint-disable MD013 -->

```sh
exonum-timestamping generate-config example/common.toml  example/node1 --peer-address 127.0.0.1:6331 --consensus-key-pass pass:a --service-key-pass pass:a
exonum-timestamping generate-config example/common.toml  example/node2 --peer-address 127.0.0.1:6332 --consensus-key-pass pass:a --service-key-pass pass:a
exonum-timestamping generate-config example/common.toml  example/node3 --peer-address 127.0.0.1:6333 --consensus-key-pass pass:a --service-key-pass pass:a
exonum-timestamping generate-config example/common.toml  example/node4 --peer-address 127.0.0.1:6334 --consensus-key-pass pass:a --service-key-pass pass:a
```

Note that in case of copying files with consensus and service keys to the other machines, you must change the access permissions of these files for every machine.
For example:

```sh
sudo chmod 600 consensus_1.toml
sudo chmod 600 service_1.toml
```

Finalize generation of nodes configurations:

```sh
exonum-timestamping finalize --public-api-address 0.0.0.0:8200 --private-api-address 0.0.0.0:8091 example/node1/sec.toml example/node1/cfg.toml --public-configs example/node1/pub.toml example/node2/pub.toml example/node3/pub.toml example/node4/pub.toml
exonum-timestamping finalize --public-api-address 0.0.0.0:8201 --private-api-address 0.0.0.0:8092 example/node2/sec.toml example/node2/cfg.toml --public-configs example/node1/pub.toml example/node2/pub.toml example/node3/pub.toml example/node4/pub.toml
exonum-timestamping finalize --public-api-address 0.0.0.0:8202 --private-api-address 0.0.0.0:8093 example/node3/sec.toml example/node3/cfg.toml --public-configs example/node1/pub.toml example/node2/pub.toml example/node3/pub.toml example/node4/pub.toml
exonum-timestamping finalize --public-api-address 0.0.0.0:8203 --private-api-address 0.0.0.0:8094 example/node4/sec.toml example/node4/cfg.toml --public-configs example/node1/pub.toml example/node2/pub.toml example/node3/pub.toml example/node4/pub.toml
```

Run nodes:

```sh
exonum-timestamping run --node-config example/node1/cfg.toml --db-path example/db1 --public-api-address 0.0.0.0:8200  --consensus-key-pass pass:a --service-key-pass pass:a
exonum-timestamping run --node-config example/node2/cfg.toml --db-path example/db2 --public-api-address 0.0.0.0:8201  --consensus-key-pass pass:a --service-key-pass pass:a
exonum-timestamping run --node-config example/node3/cfg.toml --db-path example/db3 --public-api-address 0.0.0.0:8202  --consensus-key-pass pass:a --service-key-pass pass:a
exonum-timestamping run --node-config example/node4/cfg.toml --db-path example/db4 --public-api-address 0.0.0.0:8203  --consensus-key-pass pass:a --service-key-pass pass:a
```

<!-- markdownlint-enable MD013 -->

Install frontend dependencies:

```sh
cd ../frontend

npm install
```

Build sources:

```sh
npm run build
```

Run the application:

```sh
npm start -- --port=2268 --api-root=http://127.0.0.1:8200
```

`--port` is a port for Node.JS app.

`--api-root` is a root URL of public API address of one of nodes.

Ready! Find demo at [http://127.0.0.1:2268](http://127.0.0.1:2268).

## License

Timestamping demo is licensed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
