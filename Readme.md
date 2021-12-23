# [Udemy] Building web APIs with Rust (beginners) [ENG, 2020]


<br/>

**Original Src:**  
https://gitlab.com/udemy-paris/rocket-app

<br/>

## 02. Routing

<br/>

### 01. Hello world!


```
$ mkdir app 
$ cd app
$ cargo init . --bin
$ vi ./
```

<br/>

```
$ cargo run
```

<br/>

```
$ curl localhost:8000
```

<br/>

### 02. JSON

<br/>

### 03. CRUD routes

<br/>

```
$ curl localhost:8000/rustaceans
```

<br/>

```
$ curl localhost:8000/rustaceans/1
```

<br/>

```
$ curl localhost:8000/rustaceans -X POST -H 'Content-type: application/json'
```

<br/>

```
$ curl localhost:8000/rustaceans/1 -X PUT -H 'Content-type: application/json'
```

<br/>

```
$ curl localhost:8000/rustaceans/1 -X DELETE -I
```

<br/>

### 05. Error catchers

<br/>

## 03. Auth

<br/>

### 01. Basic auth intro

<br/>

### 02. Implementing a basic auth guard

<br/>

```
$ curl localhost:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ'
```

<br/>

## 04. Database

<br/>

### 01. Diesel CLI

```
$ sudo apt install libsqlite3-dev
```

<br/>

```
$ cargo install diesel_cli --no-default-features --features sqlite
$ diesel setup --database-url=database.sqlite
$ diesel migration generate create_rustaceans
```

<br/>

```
$ diesel migration run --database-url=database.sqlite
$ diesel migration redo --database-url=database.sqlite
```

<br/>

### 02. Diesel and rocket - Dependencies


<br/>

### 03. Diesel and rocket - Model & first query

<br/>

### 04. Diesel and rocket - New model & create endpoint


<br/>

```
// CREATE
$ curl localhost:8000/rustaceans \
  -H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ" \
  -H "Content-type: application/json" \
  -X POST -d '{"name" : "John Doe", "email" : "john.doe@gmail.com"}' 
```

<br/>

```
// GET ALL
$ curl localhost:8000/rustaceans \
  -H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ" \
  -H "Content-type: application/json" \
  | jq
``` 

**returns:**

```
[
  {
    "id": 1,
    "name": "John Doe",
    "email": "john.doe@gmail.com",
    "created_at": "2021-12-20 23:58:52"
  }
]
```

<br/>

### 06. Diesel and rocket - Full CRUD

<br/>

```
// UPDATE
$ curl localhost:8000/rustaceans/1 \
  -H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ" \
  -H "Content-type: application/json" \
  -X PUT -d '{"id" : 1, "name" : "Jane Doe", "email" : "john.doe@gmail.com"}'
```

<br/>

```
// DELETE
$ curl localhost:8000/rustaceans/1 \
  -H "Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ" \
  -H "Content-type: application/json" \
  -X DELETE 
```

<br/>

### 07. Repositories


<br/>

### 09. Error handling

<br/>

### 10. Embedding migrations

<br/>

## 05. Deploying

<br/>

### 01. Deploying and systemd

```
$ cargo build --release
$ ./target/release/rocket-app
```

<br/>

```
$ ROCKET_DATABASES={sqlite_path={url./database.sqlite}} ./app
```

<br/>

**Add app as systemd service**

<br/>

```
$ sudo vi /etc/systemd/system/rocket-app.service
```

<br/>

```
[Unit]
Description=My Rocket application

[Service]
User=www-data
Group=www-data
# The user www-data should own this directory
WorkingDirectory=/var/www/rocket-app
Environment="ROCKET_ENV=prod"
Environment="ROCKET_ADDRESS=127.0.0.1"
Environment="ROCKET_PORT=8000"
Environment="ROCKET_DATABASES={sqlite_path={url./database.sqlite}}"
ExecStart=/var/www/rocket-app/rocket-app

[Install]
WantedBy=multi-user.target
```

<br/>


```
$ sudo systemctl enable rocket-app.service
$ sudo systemctl start  rocket-app.service
$ sudo systemctl status rocket-app.service
```

<br/>

### 02. Reverse proxying with nginx and ssl