{pkgs, ...}: {
  packages = with pkgs;
    [
      cargo-nextest
      easel
      infernal
      just
    ]
    ++ (with pkgs.darwin.apple_sdk; [
      frameworks.CoreFoundation
      frameworks.CoreServices
      frameworks.SystemConfiguration
    ]);

  languages = {
    rust.enable = true;
  };

  pre-commit.hooks = {
    shellcheck.enable = true;
    clippy.enable = true;
    rustfmt.enable = true;
    cargo-check.enable = true;
    alejandra.enable = true;
  };

  enterTest = ''
    cargo nextest run
  '';
}
