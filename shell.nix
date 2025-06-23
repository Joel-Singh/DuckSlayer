{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    rustup
  ];
  buildInputs = [
    udev alsa-lib-with-plugins vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
    pkgsCross.mingwW64.stdenv.cc # From https://discourse.nixos.org/t/cross-compiling-to-mingw-missing-libraries/31849
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

  CARGO_TARGET_X86_64_PC_WINDOWS_GNU_RUSTFLAGS =
    "-L native=${pkgs.pkgsCross.mingwW64.windows.pthreads}/lib"; # From https://discourse.nixos.org/t/cross-compiling-to-mingw-missing-libraries/31849

  CARGO_FEATURE_PURE=1; # For blake3 workaround https://bevy-cheatbook.github.io/setup/cross/linux-windows.html#bevy-caveats
}
