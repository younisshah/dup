## Dup

`dup::Dup::new(fd)` returns a RW pipe for the given `fd`.

#### Install

In your `Cargo.toml` file add under `[dependencies]` section

```ini
[dependencies]
dup = "0.1.0"
```

#### Example

```rust
let mut rw = dup::Dup::new(1);

thread::spawn(|| {
    println!("Hello World");
});

let mut reader = BufReader::new(rw.0);
let mut s = String::new();
reader.read_to_string(&mut s);

rw.1.write(s.as_bytes());
```


### License
MIT