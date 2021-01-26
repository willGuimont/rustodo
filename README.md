# rustodo
## Create sqlite database
```shell
echo DATABASE_URL=todos.sqlite > .env
diesel migration run
```
## Run
```shell
cargo run
```
