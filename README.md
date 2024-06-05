# Setup

Setup the database
```
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/app_db
export ROCKET_DATABASES={postgres={url=postgres://postgres:postgres@localhost:5432/app_db}}
```

Run the migrations
```

diesel setup


diesel migration run
diesel migration revert
```
