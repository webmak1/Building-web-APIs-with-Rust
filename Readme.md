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