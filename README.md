## uma_app 起動方法

### 初回起動

```
$ cd uma_app
$ docker-compose build
$ docker-compose run --rm rust diesel setup
$ docker-compose run --rm rust diesel migration run
$ docker-compose up -d
```

### 2 回目以降

```
$ cd uma_app
$ docker-compose build
$ docker-compose up -d
```
