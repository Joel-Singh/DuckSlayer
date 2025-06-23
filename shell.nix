{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    rustfmt
    cargo
    rustc
    rust-analyzer
  ];
  buildInputs = [
    udev alsa-lib-with-plugins vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
    pkgsCross.mingwW64.stdenv.cc
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

  CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS =
    "-L native=${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib";

  CARGO_FEATURE_PURE=1;
}
