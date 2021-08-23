{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [
    cargo
    pkgconfig
    udev
    alsaLib
    lutris
    x11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers
  ];
}
