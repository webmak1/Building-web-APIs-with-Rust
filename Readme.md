# [Udemy] Building web APIs with Rust (beginners) [ENG, 2020]


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