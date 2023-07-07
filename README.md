# Small Silicon Data Environment

I want to come up with a nix based local data development environment for Apple Silicon Based machines.

## Components

- Infrastructure
    - Postgres
    - Kafka


- Programming Environment
    - Python
        - Poetry
        - Jupyterlabs
        - Tensorflow preinstalled inclusive GPU Support
    - Rust
    - Scala

## HOW-TO

1. Install nix-shell
2. start nix-shell

Switch to the respective dir and run either `poetry run jupyter-lab` or `cargo run`

To start kafka console producer run `kafka-console-producer --bootstrap-server localhost:9092 --topic test`


:warning: WARNING: This is still a work in progress :warning:
