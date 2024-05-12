# Dataverse Rust

This is a Rust client for the Dataverse API. It is a work in progress and is not yet feature complete.

# File: client.rs

This file defines the `BaseClient` struct and its associated methods.

## Struct: BaseClient

This struct represents a client that can make HTTP requests. It contains the base URL for requests, an optional API token, and a `Client` instance from the `reqwest` library.

### Method: new

This method creates a new `BaseClient`. It takes a base URL and an optional API token as parameters.

### Method: get

This method sends a GET request to the specified path. It takes a path and an optional map of parameters as arguments.

### Method: post

This method sends a POST request to the specified path. It takes a path, an optional map of parameters, and a body as arguments.

### Method: put

This method sends a PUT request to the specified path. It takes a path, an optional map of parameters, and a body as arguments.

# File: info.rs

This file contains a function for getting the version of the API.

## Function: get_version

This function retrieves the version of the API. It uses the `get` method of the `BaseClient` to send a GET request to the "api/info/version" endpoint.

### Parameters

- `client: &BaseClient`: A reference to the base client used for making HTTP requests.

### Returns

- `Result<Response<VersionResponse>, String>`: This function returns a `Result` type. On success, it returns a `Response` with a `VersionResponse` body. On failure, it returns a `String` representing the error.

### Error Handling

Errors are handled by returning a `Result` type. If an error occurs, the function will return `Err` with a string representation of the error.

# File: collection.rs

This file contains a function for creating a new collection in a dataverse.

## Function: create

This function creates a new collection in a dataverse.

### Parameters

- `client: &BaseClient`: A reference to the base client used for making HTTP requests.
- `parent: &String`: A reference to the string representing the parent dataverse in which the collection will be created.
- `body: &CreateBody`: A reference to the body of the request for creating a collection.

### Returns

- `Result<Response<CreateResponse>, String>`: This function returns a `Result` type. On success, it returns a `Response` with a `CreateResponse` body. On failure, it returns a `String` representing the error.

### Error Handling

Errors are handled by returning a `Result` type. If an error occurs, the function will return `Err` with a string representation of the error.
