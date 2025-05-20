if not exist "www\pkg" mkdir www\pkg
rustup target add wasm32-unknown-unknown
wasm-pack build --release --out-dir www\pkg
if errorlevel 1 cargo install wasm-pack
wasm-pack build --release --out-dir www\pkg
cd www
call npm install
npm start
