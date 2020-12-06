# rust-python-examples
Ways for Rust and Python to play nice.

For the below examples, I'm assuming you have both Rust and Python 3 installed on your system.

## cffi

First enter the `cffi` directory. Install the requirements (just `cffi`).

Now, enter the `adder` directory and build our crate to a dynamic library:

```
cargo build --release
```

Now, check the path `location` in `add.py` is correctly pointing to the location of your built library. If so, you can run `python3 add.py`. If you have no assertion error, you are successfully calling Rust code from Python.

