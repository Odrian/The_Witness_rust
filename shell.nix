# { pkgs ? import <nixpkgs> { overlays = [ (import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz)) ]; } }:
{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  myBuildInputs = [
    # X11
    pkgs.xorg.libX11
    pkgs.xorg.libxcb
    pkgs.xorg.libXcursor
    pkgs.xorg.libXrandr
    pkgs.xorg.libXi
    pkgs.pkg-config

    # OpenGL
    pkgs.libGL
    pkgs.libGLU

    # Wayland
    pkgs.wayland
    pkgs.wayland-protocols
    pkgs.libxkbcommon
    pkgs.wlroots
    pkgs.glib
  ];
in
mkShell {
  buildInputs = myBuildInputs;

  shellHook = ''
      export LD_LIBRARY_PATH=/run/opengl-driver/lib/:${lib.makeLibraryPath myBuildInputs}:$LD_LIBRARY_PATH
  '';
}
