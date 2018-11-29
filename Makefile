NAME=jenq
VERSION=$(shell git rev-parse HEAD)
SEMVER_VERSION=$(shell grep version Cargo.toml | awk -F"\"" '{print $$2}' | head -n 1)
REPO=clux

musl:
	docker run \
		-v cargo-cache:/root/.cargo \
		-v "$$PWD:/volume" -w /volume \
		--rm -it clux/muslrust:stable cargo build --release --bin jenq
	cp target/x86_64-unknown-linux-musl/release/jenq jenq.x86_64-unknown-linux-musl
	chmod +x jenq.x86_64-unknown-linux-musl

test:
	cargo test

clippy:
	cargo clippy -- --allow clippy::if_let_redundant_pattern_matching

doc:
	cargo doc --all --all-features
	xdg-open target/doc/jenq/index.html

# Package up all built artifacts for ghr to release
#
# releases/
# ├── jenq.sha256
# ├── jenq.x86_64-apple-darwin.tar.gz
# └── jenq.x86_64-unknown-linux-musl.tar.gz
releases:
	make release-x86_64-unknown-linux-musl
	make release-x86_64-apple-darwin
	(cd releases; shasum -a 256 *.tar.gz | tee "jenq.sha256")

# Package a jenq.$* up with complete script in a standard folder structure:
#
# -rw-r--r-- user/user      5382 2018-04-21 02:43 share/jenq/jenq.complete.sh
# -rwxr-xr-x user/user         0 2018-04-21 02:43 bin/jenq
#
# This should be extractable into /usr/local/ and just work.
release-%:
	mkdir -p releases/$*/bin
	mkdir -p releases/$*/share/jenq
	cp jenq_cli/jenq.complete.sh releases/$*/share/jenq
	cp jenq.$* releases/$*/bin/jenq
	chmod +x releases/$*/bin/jenq
	cd releases && tar czf jenq.$*.tar.gz --transform=s,^$*/,, $$(find $*/ -type f -o -type l)
	tar tvf releases/jenq.$*.tar.gz
	rm -rf releases/$*/
