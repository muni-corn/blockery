{ pkgs ? import <nixpkgs> { } }:
let
  rust = pkgs.rust-bin.nightly.latest.default;
in
with pkgs;
mkShell {
  buildInputs = [
    rust

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
