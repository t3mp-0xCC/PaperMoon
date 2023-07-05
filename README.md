# PaperMoon
It's Only a Paper Moon...  

## Initialize
1. Setup `.env` file
e.g.
```python
POSTGRES_USER=postgres
POSTGRES_PASSWORD=password
POSTGRES_DB=blog
POSTGRES_HOST=127.0.0.1
DATABASE_URL="postgresql://${POSTGRES_HOST}/${POSTGRES_DB}?user=${POSTGRES_USER}&password=${POSTGRES_PASSWORD}"
TZ=Asia/Tokyo
```

2. Running Docker Containers
```
$ docker-compose up -d
```

3.  Setup Database using diesel
```
$ diesel migration run
```
If you don't have `diesel` command...  
```
$ cargo install diesel_cli
```

