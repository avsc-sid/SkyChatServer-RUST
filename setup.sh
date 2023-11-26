cp .env.example .env
rustup update
cargo clean
cargo install sqlx-cli
sqlx database create
sqlx migrate run
cargo build -r
echo "Make sure you edit your .env file. Most important to edit:

  ROCKET_SECRET_KEY: run \"openssl rand -base64 32\" and set secret key to the command output
  STATIC_PATH: set the static path to the file path to the folder containing your static files
  DOMAIN: set the domain to the domain name you are running this application on"
