{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [
    cargo
    clang
    glibc
    lld
    pkgconfig
    udev
    alsaLib
    lutris
    xlibsWrapper
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
  ];
}
