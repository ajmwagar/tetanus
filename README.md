# tetnus
:shell: A proof-of-concept reverse-shell written in rust.

## Features

- ZERO dependinces (other than rust and cargo)
- Linux Support
- Single binary
- Fast


## Compiling binary

1. `git clone https://github.com/ajmwagar/tetnus`
1. `cd tetnus`
1. Install `cargo`
2. Change Port and IP values in `src/main.rs`
3. `cargo build`
4. retrieve binary from `target/debug/tetnus`
5. Profit!

## Listen for shell

Two options for listening for connection.

1. Use netcat

```
nc -l -p <port>
```

2. Use [boa](https://github.com/ajmwagar/boa)

```
sudo boa

select 4

enter the port
```

## Roadmap

- **Windows support**
- **Code obfuscation**
- **Color support**

## Contributing

*Feel free to help*

- Open a pull request
- Submit an issue
- Share with friends!
