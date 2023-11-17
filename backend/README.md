## 📄⚠️ Important note ⚠️📄

Later, an autonomus bash file will be created to make all this config simplier. For now this is the way to go: **manually**.

# Creation of the Database

`[name]` being the username you created in postgres\
`[pwd]` being its password

### 🐘 Make sure you installed Postgresql 🐘

#### For Fedora

```bash
sudo dnf install postgresql postgresql-server postgresql-contrib
sudo /usr/bin/postgresql-setup --initdb
```

### ⚠️ Be sure to modify your config file to access the postgres database ⚠️

```bash
sudo nano /var/lib/pgsql/data/pg_hba.conf
```
Then check if those lines are correct:
```
# IPv4 local connections:
host    all             all             127.0.0.1/32            md5
# IPv6 local connections:
host    all             all             ::1/128                 md5
```
### _If modified, restart psql service :_
```
sudo service postgresql restart
```

### 💻 Create a new user and the database 💻

```sql
sudo -u postgres psql -U postgres
CREATE USER [name] WITH PASSWORD '[pwd]';
CREATE DATABASE cv OWNER [name];
GRANT ALL PRIVILEGES ON DATABASE cv TO [name];
\q
sudo service postgresql restart
```

### ⬆️ Import the sql tables ⬆️

In the `backend` directory you can find a `db` folder which contains all the tables structures for the data base. To import those definitions you can type this command:

```bash
for file in backend/db/*.sql; do
  psql -U [name] -d cv -a -f "$file"
done
```

## Env file configuration

Create the `.env` file in this folder root. The file must be composed of the following line:

```
DATABASE_URL=postgres://[name]:[pwd]@localhost/cv
```