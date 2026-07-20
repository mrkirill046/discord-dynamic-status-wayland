{
  config,
  lib,
  pkgs,
  inputs,
  ...
}:

let
  cfg = config.services.dynamic-drpc-wayland;

  tomlFormat = pkgs.formats.toml { };

  configFile = tomlFormat.generate "config.toml" {
    settings = cfg.settings;
    default = cfg.default;
    classes = cfg.classes;
  };
in
{
  options.services.dynamic-drpc-wayland = {
    enable = lib.mkEnableOption "Discord Dynamic Status Wayland";

    settings = {
      app_id = lib.mkOption {
        type = lib.types.str;
        default = "1460605258072985705";
        description = "Discord application ID";
      };

      wm = lib.mkOption {
        type = lib.types.enum [
          "niri"
          "hyprland"
        ];

        default = "niri";

        description = "Wayland compositor";
      };

      update_delay = lib.mkOption {
        type = lib.types.int;
        default = 3;
        description = "RPC debounce delay";
      };
    };

    default = lib.mkOption {
      type = lib.types.attrsOf lib.types.anything;

      default = {
        state = "Chilling";
        details = "At the workspace";

        large_text = "{pretty_os}";
        large_image = "{os}";

        small_text = "{wm}";
        small_image = "{wm}";
      };
    };

    classes = lib.mkOption {
      type = lib.types.attrsOf (lib.types.attrsOf lib.types.anything);

      default = {
        ghostty = {
          match = "com.mitchellh.ghostty";

          state = "At ghostty";
          details = "Writing command lines";
          small_text = "Ghostty with Fish";
        };

        kitty = {
					state = "At kitty";
					details = "Writing command lines";
					small_text = "Kitty with Fish";
        };

        code = {
          state = "At VSCode";
          details = "Developing some programs";
          small_text = "VSCode";
        };
      };
    };
  };

  config = lib.mkIf cfg.enable {
    home.packages = [
      inputs.dynamic-drpc-wayland.packages.${pkgs.stdenv.hostPlatform.system}.default
    ];

    xdg.configFile."dynamic-drpc-wayland/config.toml".source = configFile;

    systemd.user.services.dynamic-drpc-wayland = {
      Unit = {
        Description = "Discord Dynamic Status Wayland";

        After = [
          "graphical-session.target"
        ];
      };

      Service = {
        ExecStart = "${
          inputs.dynamic-drpc-wayland.packages.${pkgs.stdenv.hostPlatform.system}.default
        }/bin/discord-dynamic-status-wayland";
        Restart = "on-failure";
        RestartSec = 3;
      };

      Install = {
        WantedBy = [
          "graphical-session.target"
        ];
      };
    };
  };
}
