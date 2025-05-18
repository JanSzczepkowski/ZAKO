# Polish version

## Wstęp

Projekt ten ma na celu pokazanie w jaki sposób połączyć języki GO oraz Rust z językiem C. Został opracowany w ramach przedmiotu `Zaawansowane architektury komputerów` na studiach.

## Jak pracować z projektem?

Aby zacząć pracę z projektem należy skopiować repozytorium standardu [libiec61850](https://github.com/mz-automation/libiec61850). Następnie w przypadku języka GO podmienić foldery [examples/sv_publisher](https://github.com/mz-automation/libiec61850/tree/v1.6/examples/sv_publisher) oraz [examples/sv_subscriber](https://github.com/mz-automation/libiec61850/tree/v1.6/examples/sv_subscriber) na `publisher` oraz `subscriber` znajdujące się w ścieżce `go/`, a dla języka Rust do obecnie istniejących folderów [examples/sv_publisher] oraz [examples/sv_subscriber] dodać pliki znajdujące się w folderach `publisher` oraz `subscriber` w ścieżce `rust/`.

# English version

## Introduction

This project aims to demonstrate how to integrate the GO and Rust programming languages with the C language. It was developed as part of the course `Advanced Computer Architectures` during university studies.

## How to work with the project?

To get started, clone the repository of the [libiec61850](https://github.com/mz-automation/libiec61850) library. Then, for GO replace the [examples/sv_publisher](https://github.com/mz-automation/libiec61850/tree/v1.6/examples/sv_publisher) and [examples/sv_subscriber](https://github.com/mz-automation/libiec61850/tree/v1.6/examples/sv_subscriber) folders with the `publisher` and `subscriber` directories located in the `go/`. For Rust add files from `publisher` and `subscriber` folders located in the `rust/` path to the current existing [examples/sv_subscriber] and [examples/sv_publisher].
