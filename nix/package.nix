{
  lib,
  rustPlatform,
  pkg-config,
}:

rustPlatform.buildRustPackage {
  pname = "discord-dynamic-status-wayland";
  version = "2.0.0";

  src = lib.cleanSource ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs = [
    pkg-config
  ];

  meta = with lib; {
    description = "Dynamic Discord Rich Presence based on active Hyprland / Niri windows";
    homepage = "https://github.com/mrkir/discord-dynamic-status-wayland";
    license = licenses.mit;
    platforms = platforms.linux;
    mainProgram = "discord-dynamic-status-wayland";
  };
}
