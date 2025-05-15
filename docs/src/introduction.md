# Introduction

Welcome to Rust Dataverse!

This library provides a CLI for interacting with Dataverse, as well as a Rust library for building Dataverse-compatible applications. It features a broad range of functionality, including:

- Creating and managing Dataverse collections and datasets
- Uploading and managing files in Dataverse
- Searching for datasets and files
- Downloading files from Dataverse
- Direct upload of files to Dataverse
- Functionalities to remain uploads and downloads resilient to network failures

Rust Dataverse is built for performance, reliability, and scalability. It features a comprehensive test suite to ensure that the library behaves as expected in a wide range of scenarios. Compared to it's Python counterparts, it makes use of Rust's concurrency features to handle uploads and downloads in parallel more efficiently, while maintaining a similar level of ease of use.