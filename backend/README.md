# Creation of the Database

`[name]` being the username you created in postgres\
`[pwd]` being its password

```
sudo dnf install postgresql postgresql-client postgresql-server postgresql-contrib
sudo -u postgres createdb --owner=name cv
sudo service postgresql restart
```

## Importing `.sql` files

Idk for now (xd)

## ⚠️ Be sure to modify your config file to access the postgres database ⚠️

```
sudo nano /var/lib/pgsql/data/pg_hba.conf
```
Check if those lines are correct :
```
# IPv4 local connections:
host    all             all             127.0.0.1/32            md5
# IPv6 local connections:
host    all             all             ::1/128                 md5
```

# Env file configuration

```
DATABASE_URL=postgres://[name]:[pwd]@localhost/cv
```