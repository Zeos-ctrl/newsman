# Maintainer: Connor Bryan <connor.bryan@zeos-systems.com>

pkgname='newsman'
pkgver=1.0.0
pkgrel=1
pkgdesc="Mailing list manager"
arch=('x86_64')
url="https://github.com/Zeos-ctrl/newsman"
license=('GPL')
depends=(mariadb)
makedepends=(
    cargo
    git
)
source=("$pkgname-$pkgver.tar.gz::https://github.com/Zeos-ctrl/$pkgname")
md5sums=('SKIP')

prepare() {
        export RUSTUP_TOOLCHAIN=stable
        cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
        export RUSTUP_TOOLCHAIN=stable
        export CARGO_TARGET_DIR=target
        cargo build --frozen --release --all-features
}

check() {
        export RUSTUP_TOOLCHAIN=stable
        cargo test --frozen --all-features
}

package() {
        install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}

