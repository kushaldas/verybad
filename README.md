# Very Bad Web application

This is an example web application written in Rust, which contains a lot of
security issues. Do not run this in any network connected system, and use it
only in VMs to learn about security implications of bad code.



## API


- GET /filename -> will provide the file from the current directory.
- GET /getos -> will give you details about your server.
- GET /exec/date -> will tell you the time on the server.
- POST /filename -> will save the input data in `/tmp/filename` and returns `Okay`.


## Building and running

```
cargo build
cargo run
```


Then on another terminal:

```
$ curl http://localhost:8000/exec/date
Tue Mar 15 07:52:47 AM CET 2022
```

## LICENSE

MIT