# rust-groceries-mongo-api

Sample Rust project which shows how to easily expose a REST API using the [Warp](https://docs.rs/warp/) web framework, to provide a simple _Groceries_ stock management example scenario. This is based on the commonly referenced [Creating a REST API in Rust with warp](https://blog.logrocket.com/creating-a-rest-api-in-rust-with-warp/) blog post, but refactored to use a MongoDB database as a backing store, rather than the in-memory _HashMap_ used in the blog post. In this project, the [MongoDB Rust Driver](https://docs.rs/mongodb/) is used (in its default _async_ mode) and the [Rust Serde](https://serde.rs/) framework is utilised to easily convert BSON returned from MongoDB into Rust types and then into JSON to be returned by the REST API calls.

## Building & Running The Project

_(ensure you've cloned/copied this GitHub project first to your local machine, and that you have an accessible MongoDB database running locally or remotely)_

 1. Install the latest version of the [Rust development environment](https://www.rust-lang.org/tools/install), if it isn't already installed, via the __rustup__ utility, including the _rustc_ compiler & the _cargo_ package/build manager. _NOTE:_ If building on Windows 10, first ensure you have Microsoft's [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16) installed (and importantly, when running Micosrosoft's _build tools_ installer, choose the _C++ build tools_ option)

 2. From a terminal/prompt/shell, from this project's root folder, run Rust's _cargo_ command to build the project and run the debug version of the application which will start listening for Grocery REST API requests, as shown in the example below (change the URL to match the specific MongoDB database deployment target you want to test):
 
```console
cargo build
# Example connnecting to locally running MongoDB database
cargo run -- 'mongodb://localhost:27017'
# Example connnecting to a remote MongoDB Atlas clustered database
cargo run -- 'mongodbv+srv://myusr:pswd@mycluster.a113z.mongodb.net'
```

&nbsp;_OPTIONAL_: Build an _executable_ version of the application and then run it:
```console
cargo build --release
target/release/rust-groceries-mongo-api 'mongodb://localhost:27017'
```

## Testing The REST API Via The Command Line

From a terminal/prompt/shell, the following commands can be run to test the running REST API Groceries service (you can also run the `./test.sh` script provided in this project, which automates these tests)...

&nbsp;__Insert 3 apples:__
```console
curl -sS --location --request POST 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apples",
    "quantity": 3
}'
```

&nbsp;__List all the groceries in stock:__
```console
curl -sS --location --request GET 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'
```

&nbsp;__Add 5 more apples:__
```console
curl -sS --location --request PUT 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apples",
    "quantity": 5
}'
```

&nbsp;__Delete all the apples:__
```console
curl --location --request DELETE 'localhost:8080/v1/groceries' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apples"
}'
```

