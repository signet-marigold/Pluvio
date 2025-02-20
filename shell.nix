let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      dbus
      gtk3
      atk
      webkitgtk_4_1
      openssl
      alsa-lib
    ];
    nativeBuildInputs = with pkgs; [
      pkg-config
    ];
    dbus = pkgs.dbus;
    shellHook =
      ''
        export WEBKIT_DISABLE_DMABUF_RENDERER=1
      '';
  }

