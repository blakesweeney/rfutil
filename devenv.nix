{pkgs, ...}: {
  packages = with pkgs; [
    cargo-nextest
    easel
    infernal
    just
  ];

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
