{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, rust-overlay, nixpkgs}:
    let
      program_name = "lox";
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit overlays system;
      };

      rust-bin = pkgs.rust-bin.selectLatestNightlyWith
        (toolchain: toolchain.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-emscripten" ];
        });

      rust-dev-deps = with pkgs; [
        rust-analyzer
        rustfmt

        llvmPackages.clang
        llvmPackages.libclang
        llvmPackages.libstdcxxClang
        llvmPackages.libllvm
        llvmPackages.libcxx
        emscripten

        libGL

        libxkbcommon 
        wayland
        pkg-config
        clang-tools
        gcc
        cmake
        xorg.libX11
        xorg.libXcursor
        xorg.libXi
        xorg.libXrandr
        xorg.libXinerama
        libpulseaudio
      ];
      all_deps = rust-dev-deps ++ [ rust-bin ];
    in
    {
      devShell.${system} =
        pkgs.mkShell rec {
          nativeBuildInputs = [pkgs.pkg-config];
          buildInputs = all_deps;
          PROGRAM_NAME = program_name;
          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}";
            export CARGO_MANIFEST_DIR=$(pwd)
          '';
        };
    };
}
