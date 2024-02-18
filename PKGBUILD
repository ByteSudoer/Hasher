# Maintainer: ByteSudoer <soussi.mohamednour@gmail.com>
pkgname=hasher
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
license=('MIT')

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git=https://github.com/ByteSudoer/Hasher
}
