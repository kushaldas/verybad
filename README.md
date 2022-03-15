# Very Bad Web application

This is an example web application written in Rust. Do not run this in any network connected system, and use it only in VMs to learn about security
implications of bad code.



## API


- /filename -> will provide the file from the current directory.
- /getos -> will give you details about your server.
- /exec/date -> will tell you the time on the server.


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