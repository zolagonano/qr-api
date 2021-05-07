# QR-API

![b](https://img.shields.io/crates/l/qr-api)
![b](https://img.shields.io/crates/v/qr-api)

A simple and fast QRcode encoder/decoder API written in [rust](https://rust-lang.org).

## Usage:

**Notes:**
- This API takes and returns base64 only.
- Generated QRCode will be a PNG picture.
- This API takes only GET requests.
- This API is accessible through `https://qr-ende.herokuapp.com`.

### Encode:

To encode our text, we need to send a request to the `/encode/<your_text>` path, for example:

```bash
curl 'https://qr-ende.herokuapp.com/encode/hello'
```

That command above will give us a **128px in 128px** **PNG** picture of QRCode, encoded with **base64**:

- base64: 

```json
"iVBORw0KGgoAAAANSUhEUgAAAHQAAAB0CAAAAABx8Un7AAACOElEQVR4nO3NW4obQRBEUc/+F237NAQE1VWSNbjnKy+IeFS24uv3r59nRh9lRh9lRh9lRh9lRh/lNvr193cit7lJRrodfYcZ3ZLb3CQj3Y6+w3Z07dA9j2ToOoddP6M3uueRDF3nsOuPozQkU+x8a0imzYxeGpIpdr41JNPmW6OQkU7maUimzYxeGpJpkJFO5mlIps23RnnI/KohmTYzemlIpuAh86uGZNocR1fSR8FD5unKrp/RG+mj4CHzdGXXb0dPuPV+0hPemxnd4tb7SU94b26jn5ChT/9jRv+J/zbqj9LFU6w+6OToO2b0Ip5i9UEnR99xGw35Awq+0Xe3Zuhw69ci+MAbBd/ou1szdLj1axF84I2Cb/TdrRk63Pq1CD7wRsE3+u7WDB1u/Vo0/VF80CF9MrrjaTOjFz6Am/igQ/pkdMfT5jbqKHhL5iHvfJOegm9m9MJbMg9555v0FHxzG0Uf81HwkPmTvmJGL7zxUfCQ+ZO+Yjv6Cn8K3/E0yNDxrc2MHvEn8B1Pgwwd39rcRh2d6Ft3yfGtQV6Z0S196y45vjXIK9vRtUP61qY7/hUzeiN9a9Md/4rjKA3JtOmOD+mg74wZvTQk06Y7PqSDvjM+HqWIpyGZNrpmRi8NyVHE05BMG13z8Sh4yDwNyas2M3ppSKbgIfM0JK/aHEdXuudxyu+Y0Rvd8zjld2xHT7jdvevhjV91ZUa3uN296+GNX3XlNvoTzOijzOijzOijzOijzOij/AFHTEgf6TUWfAAAAABJRU5ErkJggg=="
```

- png:

![img](qr_examples/default_hello.png)

Also, we can encode our QRCode with custom width and height using query strings:

```bash
curl 'https://qr-ende.herokuapp.com/encode/hello?width=64&height=64'
```

Now that command above will give us a 64px in 64px PNG picture of QRCode, encoded with base64:

- base64: 

```json
"iVBORw0KGgoAAAANSUhEUgAAADoAAAA6CAAAAADE0BX0AAABJUlEQVR4nO3O0WoDMQxE0eb/P7r1YSrLW1JIi0JeckHSeKRZ9vb58V/e0Yd4cfS2quFyzKgm3nK3WNVwOWZUE2+5W2yrtIBJZaL1VNSxczpOegomF8+J8rxMHkwOnhP1ik7BtMNUtKAVdT3nlJ6JnuS862QntljVOD/rZCdK3Efot4uJqB9Lz3k64kWfTEThoCN5oyZ1bEvAyWX5rWpSx7ZEyEqHHW2WOj5TItQ62NFmqTtRi6x07+6Bri0mokuuslQUh+66MhO9kngF+qXCRNSyiZszPQVOmImWik6FqN6HqagDJ3SIKh9x8IworWdvchBneVssK0tTlYLJQZzlbbGsLE2uV2mvaBWmokV0oj1/MhM98WOFCx+rKiaif+cdfYjXRL8A+QUkELIXPo0AAAAASUVORK5CYII="
```

- png:

![img](qr_examples/64px_hello.png)


### Decode:

The decoding process is the opposite of the encoding process, which means we have to pass the base64 of our QRCode to the API, to decode a QRCode we need to send a request to the `/decode/<your_qrcode_base64>`:


For this example, we can use our base64 encoded QRCode that we get in encode section:

```json
"iVBORw0KGgoAAAANSUhEUgAAADoAAAA6CAAAAADE0BX0AAABJUlEQVR4nO3O0WoDMQxE0eb/P7r1YSrLW1JIi0JeckHSeKRZ9vb58V/e0Yd4cfS2quFyzKgm3nK3WNVwOWZUE2+5W2yrtIBJZaL1VNSxczpOegomF8+J8rxMHkwOnhP1ik7BtMNUtKAVdT3nlJ6JnuS862QntljVOD/rZCdK3Efot4uJqB9Lz3k64kWfTEThoCN5oyZ1bEvAyWX5rWpSx7ZEyEqHHW2WOj5TItQ62NFmqTtRi6x07+6Bri0mokuuslQUh+66MhO9kngF+qXCRNSyiZszPQVOmImWik6FqN6HqagDJ3SIKh9x8IworWdvchBneVssK0tTlYLJQZzlbbGsLE2uV2mvaBWmokV0oj1/MhM98WOFCx+rKiaif+cdfYjXRL8A+QUkELIXPo0AAAAASUVORK5CYII="
```

But we have to encode that in URLencode first:

```
iVBORw0KGgoAAAANSUhEUgAAADoAAAA6CAAAAADE0BX0AAABJUlEQVR4nO3O0WoDMQxE0eb%2FP7r1YSrLW1JIi0JeckHSeKRZ9vb58V%2Fe0Yd4cfS2quFyzKgm3nK3WNVwOWZUE2%2B5W2yrtIBJZaL1VNSxczpOegomF8%2BJ8rxMHkwOnhP1ik7BtMNUtKAVdT3nlJ6JnuS862QntljVOD%2FrZCdK3Efot4uJqB9Lz3k64kWfTEThoCN5oyZ1bEvAyWX5rWpSx7ZEyEqHHW2WOj5TItQ62NFmqTtRi6x07%2B6Bri0mokuuslQUh%2B66MhO9kngF%2BqXCRNSyiZszPQVOmImWik6FqN6HqagDJ3SIKh9x8IworWdvchBneVssK0tTlYLJQZzlbbGsLE2uV2mvaBWmokV0oj1%2FMhM98WOFCx%2BrKiaif%2BcdfYjXRL8A%2BQUkELIXPo0AAAAASUVORK5CYII%3D
```

Now we can decode our QRCode:

```bash
curl 'https://qr-ende.herokuapp.com/decode/iVBORw0KGgoAAAANSUhEUgAAADoAAAA6CAAAAADE0BX0AAABJUlEQVR4nO3O0WoDMQxE0eb%2FP7r1YSrLW1JIi0JeckHSeKRZ9vb58V%2Fe0Yd4cfS2quFyzKgm3nK3WNVwOWZUE2%2B5W2yrtIBJZaL1VNSxczpOegomF8%2BJ8rxMHkwOnhP1ik7BtMNUtKAVdT3nlJ6JnuS862QntljVOD%2FrZCdK3Efot4uJqB9Lz3k64kWfTEThoCN5oyZ1bEvAyWX5rWpSx7ZEyEqHHW2WOj5TItQ62NFmqTtRi6x07%2B6Bri0mokuuslQUh%2B66MhO9kngF%2BqXCRNSyiZszPQVOmImWik6FqN6HqagDJ3SIKh9x8IworWdvchBneVssK0tTlYLJQZzlbbGsLE2uV2mvaBWmokV0oj1%2FMhM98WOFCx%2BrKiaif%2BcdfYjXRL8A%2BQUkELIXPo0AAAAASUVORK5CYII%3D'
```

The output of the command above will be `hello`.

## Run it by yourself:

If you want to run this API by yourself, you have two options(maybe more than two):

- Deploy to [Heroku](https://heroku.com).
- Run it on a VPS.

### Deploy to Heroku:

Deploying to Heroku is the simplest way to run this API, you only need an account in `heroko.com`:

1. Login to your Heroku account using `heroku-cli`:

```bash
heroku login
```

2. Clone this repository:

```bash
git clone https://github.com/zolagonano/qr-api.git
```

3. Add Heroku to your remotes using `heroku-cli`:

```bash
heroku git:remote -a <your-app-name>
```

4. Deploy your API:

```bash
git push heroku master
```

### Run it on a VPS:

- If you want to run this project on a VPS, you need rustup with the nightly toolchain.
- To install rustup and nightly toolchain you can check [rustup.rs](https://rustup.rs).

### Install from the source code:

1. Clone this repository:

```bash
git clone https://github.com/zolagonano/qr-api.git
```

2. Change your working directory to source code directory:

```bash
cd qr-api
```

3. Run the API:

```
cargo run --release
```

### Install from cargo:

1. Install it:

```bash
cargo install qr-api
```

2. Run it:

```bash
qr-api
```

## Contribute:

All contributions are welcome but if you don't know what you can do look at this list:

- Open an issue if you find a bug.
- Open an issue if you have a suggestion.
- Fix bugs and send pull requests.
- Share it with your friends.
- And anything else you think will help this project :).

## License

Licensed under MIT license.
