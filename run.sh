concurrently "cd  ./backend && cargo build" "cd ./app/web && npm install" "cd ./app/extension && npm install" "cd ./app/extension && npm install"
concurrently "cd  ./backend && nodemon" "cd ./app/web && npm run dev" "cd ./app/extension && npm run dev:firefox" "cd ./app/extension && nodemon"
