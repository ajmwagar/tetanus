# tetnus
:shell: A proof-of-concept reverse-shell written in rust.

## DISCLAMER

**For educational purposes only!**

**I am not responsible for damage or legal trouble you may incur from using tetnus.**

**Use at your own risk and only on networks you have permission to access.**


## Features

- ZERO dependencies (other than winApi on windows)
- MacOS/Linux Support (Windows coming soon)
- Compiles to machine code.
- Fast
- Recursive connecting


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
- **Reboot persitence**
- **C2 Server (for now see above)

## Contributing

*Feel free to help*

- Open a pull request
- Submit an issue
- Share with friends!
