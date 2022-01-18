# server-api
L'api du projet

## Lancer le docker
```bash
docker-compose up --build
```
## Installer Rust
### Windows
Installer visual studio avec les option C++

allez sur ce site : 
https://www.rust-lang.org/tools/install

Téléchargez l'installeur et executer le.

### Linux et Mac

executer : 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Si cela ne fonctionne pas. allez sur ce site : 
https://www.rust-lang.org/tools/install

## Lancer les différente version
### Debug
``` bash
cargo run
```
### Test
```bash
cargo test
```
### Release
```bash
cargo run --release
```

## Orm - Diesel
### installer diesel
#### Requirement 
Avoir la library de dev postgres installer

#### Installation
```bash
sudo apt-get install libssl-dev postgresql-12
cargo install diesel_cli --no-default-features --features postgres
```
#### Utilisation
définire l'environement, test : 
```bash
echo DATABASE_URL=postgres://api-renov:api_pswd@localhost/debug > .env
```
initialiser diesel : 
```bash
diesel setup
```
Créer une migration :
```bash
diesel migration generate <migration name>
```
Lancer la migration :
```bash
diesel migration run
```