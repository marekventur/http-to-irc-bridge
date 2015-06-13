DESTDIR ?=

.EXPORT_ALL_VARIABLES:

default: all

all: clean build

build:
	cargo build --release

#rustup:
#	curl -L https://static.rust-lang.org/rustup.sh | bash -s -- --prefix=$(DESTDIR)/tmp/ --disable-sudo

test:
	cargo test

install:
	echo "DESTDIR $DESTDIR"
	mkdir -p $(DESTDIR)/usr/bin
	install -m 0755 target/release/httptoircbridge $(DESTDIR)/usr/bin/httptoircbridge

deb:
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-ignore-new --git-pbuilder

local-deb:
	debuild --preserve-env --prepend-path=/usr/local/bin -d binary

release:
	git-dch -a -c -R --full --debian-tag="v%(version)s"
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-pbuilder --git-tag --git-debian-tag="v%(version)s"

clean:
	rm -rf target
	rm -rf debian/httptoircbridge

snapshot:
	git-dch -a -S --full --debian-tag="v%(version)s"
	git-buildpackage --git-upstream-branch=master --git-debian-branch=master --git-ignore-new --git-pbuilder  --git-debian-tag="v%(version)s"
