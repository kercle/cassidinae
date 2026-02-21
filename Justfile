[working-directory: 'web/frontend']
serve-frontend-dev:
    npm install
    npm run dev

eval-server:
    cargo run --bin eval-server

[parallel]
serve-dev: eval-server serve-frontend-dev
