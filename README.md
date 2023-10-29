# Rust-Webserver

A very simple implementation of a webserver in rust.

# Disclaimer

-   ⚠️ The project was designed for fun and learning, it should **NOT** be used for production purposes.
-   ⚠️ There are probably many bugs, and functionalities, which have not been added and never will be.

# Features

-   Threaded GET Requests
-   Serving html files
-   Serving other text based files (e.g javascript, css)
-   Serving images
-   Error Pages (e.g 404, 500)

# Usage

This setup assumes you installed Rust. If you haven't you can download it here [here](https://www.rust-lang.org/tools/install).

First clone the git repository.

```
git clone https://github.com/NLion74/Rust-Webserver
```

After having cloned the repository move into `Rust-Webserver/src` and create a repository called `html` (Can be changed by editing `src/main.rs`) in which you can now put your html files. Finally start the server using cargo.

```
cd Rust-Webserver/src

# Create a folder called "html" and put in the files you want to serve.

cargo run
```

Your server should now be up and running head to `127.0.0.1:6969` in your browser or alternatively if installed on another machine, the ip address of the machine with the port 6969. The port can be changed by editing `src/main.rs`.
