# NOTE: Please fill out the license field for your package! If it is unknown,
# then please put 'unknown'.

# Maintainer: Joseph Hendrix <joeylhendrix@gmail.com>
pkgname='rsls-git'
pkgver=1.0
pkgrel=1
pkgdesc="ls with icons, written in Rust"
arch=("x86_64")
url="https://github.com/joeleehen/rsls"
license=('GPL')
depends=(gcc-libs glibc ttf-nerd-fonts-symbols)
makedepends=(git cargo)
#source=("rsls-$pkgver.tar.gz::https://github.com/joeleehen/rsls/archive/refs/tags/v$pkgver.tar.gz")
source=('rsls::git+https://github.com/joeleehen/rsls.git#branch=master')
sha256sums=('SKIP')

build() {
    cd "rsls"
    cargo b --release
    # make
}

package() {
    echo "$srcdir"
    cd "$srcdir/rsls/target/release"
    install -Dm755 rsls "$pkgdir/usr/bin/rsls"
}
