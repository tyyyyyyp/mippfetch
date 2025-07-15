# Maintainer: Made by meepo2k17 any qeustions matviy.khomchak@gmail.com

pkgname=mippfetch
pkgver=0.1.0
pkgrel=1
pkgdesc="A minimal Rust-based system fetch tool like neofetch"
arch=('x86_64')
url="https://github.com/yourusername/mippfetch"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=()
md5sums=()

build() {
  cargo build --release --target-dir=target
}

package() {
    install -Dm755 "$srcdir/target/release/mippfetch" "$pkgdir/usr/bin/mippfetch"
    install -Dm644 "$srcdir/mippfetch.conf" "$pkgdir/etc/mippfetch/mippfetch.conf"
}


