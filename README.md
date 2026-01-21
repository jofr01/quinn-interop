# Quinn interop

Interoperability container for [Quinn](https://github.com/quinn-rs/quinn), a `QUIC` implementation in pure Rust.

## Overview

This repository contains a server and client implementation to test interoperabilty with other `QUIC` and `HTTP/3` implementations. It also provides a container definition to be integrated to [QUIC interop runner]( https://github.com/marten-seemann/quic-interop-runner ), see the results [here](https://interop.seemann.io/).


## Directory summary:

* quinn-interop: interop application to be deployed into the container

## Getting started

Start by cloning the repository with its submodules:

``` sh
git clone git://github.com/quinn-rs/quinn-interop

```

### Using your machine

First, install the runner dependencies:

``` sh
pip3 install -r quic-interop-runner/requirements.txt
```

You will need some other dependencies on your system:
* tshark
* docker-compose

``` sh
# Build the container
docker build -f Dockerfile  -t quinn-interop:0.11.9 .
```

Use it in the [QUIC interop runner]( https://github.com/marten-seemann/quic-interop-runner ) by editing the 'implementations.json' file:
``` json
  "quinn": {
    "image": "quinn-interop:0.11.9",
    "url": "https://github.com/quinn-rs/quinn",
    "role": "both"
  }
```