# Maintainer: Edward Thomas <edward@programmingeveryday.com>
pkgname=omarchy-updater-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="GTK4/Libadwaita system update dashboard for Omarchy Linux"
arch=('x86_64')
url="https://github.com/programmingeveryday/omarchy-updater"
license=('MIT')
depends=('gtk4' 'libadwaita')
provides=('omarchy-updater')
conflicts=('omarchy-updater')
source=("$pkgname-$pkgver.tar.gz::$url/releases/download/v$pkgver/omarchy-updater-linux-x86_64.tar.gz")
sha256sums=('SKIP')

package() {
  install -Dm755 "$srcdir/bin/omarchy-updater" "$pkgdir/usr/bin/omarchy-updater"
  install -Dm644 "$srcdir/share/applications/omarchy-updater.desktop" "$pkgdir/usr/share/applications/omarchy-updater.desktop"
}
