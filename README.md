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

## pyo3_basic

Enter the `pyo3_basic` directory. Install the requirements (we're using `setuptools-rust` for packaging).

Next:

```
python3 setup.py develop
```

This will call out to `rustc` to compile the extension. It'll also install the resulting Python module into your virtualenv.

You can now from a Python interpreter:

```py
>>> import adder
>>> adder.add(2, 3)
5
```

## pyo3_inherit

Enter the `pyo3_inherit` directory. You build as above in the basic example:

```
python3 setup.py develop
```

You can access methods on either the parent (`feed()`) or child (`roar()`):

```py
>>> doris = zoo.Lion('Doris', 2, 0, 'human')
>>> doris.name
'Doris'
>>> doris.age
2
>>> doris.feed()
>>> doris.roar()
'ROAR!!!!'
>>> doris.favorite_meat
'human'
```
