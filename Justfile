default:
  @just --list --justfile {{justfile()}}

set dotenv-load := true

tailwind:
  npx tailwindcss -i assets/css/input.css -o assets/css/styles.css --minify

tailwind-watch:
  just tailwind &
  
build:
  npm install
  just tailwind
  cargo build
  trunk build

build-release:
  npm install
  just tailwind
  cargo build --release

clean:
  rm -rf dist
  trunk clean
  cargo clean

test: test-unit

kill-server:
  kill $(cat .pid 2>/dev/null) 2>/dev/null || true

test-e2e: && kill-server
  trunk serve --port=5274 & pid=$!; echo "$pid" > .pid
  sleep 5
  CI=true npx cypress run
  
test-unit: build
  cargo nextest run

lint: && audit disallow-unused-cargo-deps
  cargo clippy -- -D warnings
  cargo clippy --all-targets --all-features -- -D warnings
  cargo check

test-ci: lint test-unit test-e2e

disallow-unused-cargo-deps:
  cargo machete Cargo.toml

audit:
  cargo audit

serve: tailwind-watch build 
  trunk serve --open --port=$((5170 + $RANDOM % 10))

release: build-release
  trunk build --release --filehash true

pre_build:
  mkdir -p $TRUNK_STAGING_DIR/assets/img/ $TRUNK_STAGING_DIR/assets/css/
  cp assets/img/* $TRUNK_STAGING_DIR/assets/img/
  cp assets/robots.txt $TRUNK_STAGING_DIR

publish: release
  aws cloudfront create-invalidation --distribution-id "$DIST_ID" --paths "/*"
  aws s3 cp dist "s3://$BUCKET_NAME" --recursive
