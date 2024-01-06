# Rust Based CLI Tool to TOTP(2FA) via QR from clipboard

This project allows you to read a QR code containing TOTP from your clipboard and write it to a json file by encrypting it with AES. You can view your codes at any time using the password you set. Do not hesitate to create an issue for the features you want to be added.  



## How to use?

### Installation

1. Clone this repository:
```bash
git clone https://github.com/hkey0/rust-cli-totpgit
cd rust-cli-totp
```

2. Build with cargo.
```bash
cargo build
```


### Usage

There are 2 commands available; `new` and `show`. The `new` command tries to read the QR code in your clipboard, if it succeeds it asks you for a password and saves the totp secret key in the `secrets.json` file. For the current version all your passwords have to be the same :|

The `show` command instantly generates a code for all your secret keys, this code is totp. 
```bash
cargo run -- new
cargo run -- show
```

There is a great site to test this; [2fas.com](https://2fas.com/check-token/).
![clinew](https://raw.githubusercontent.com/hkey0/rust-cli-totp/main/images/2fas.png)
All you need to do is take a screenshot of the QR code and run the code with new flag:

```bash
cargo run -- new
```

This is what you are likely to see:<br />
![clinew](https://raw.githubusercontent.com/hkey0/rust-cli-totp/main/images/clinew.png)

And now just run the `show command`:
```bash
cargo run -- show
```
![clinew](https://raw.githubusercontent.com/hkey0/rust-cli-totp/main/images/samecli.png)
![clinew](https://raw.githubusercontent.com/hkey0/rust-cli-totp/main/images/same1.png)

Here it worked!


### TODO

- [] GUI