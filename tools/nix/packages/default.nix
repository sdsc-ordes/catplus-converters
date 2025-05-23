{
  pkgs,
  lib,
  rootDir,
  rustToolchain,
  ...
}:
let
  fs = lib.fileset;

  # Define our source fileset by removing `tools` folder.
  gitTrackedFS = fs.gitTracked rootDir;
  toolsFS = fs.fromSource (rootDir + "/tools");
  src = fs.toSource {
    root = rootDir;
    fileset = fs.difference gitTrackedFS toolsFS;
  };

  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };

  cargoFile = rootDir + "/src/converter/Cargo.toml";
  # We need the workspace lock file cause we need all
  # dependencies correctly.
  lockFile = rootDir + "/Cargo.lock";
in
rustPlatform.buildRustPackage {
  name = "catplus-converters";
  inherit src;

  version = (lib.importTOML cargoFile).package.version;

  cargoLock = {
    inherit lockFile;
  };

  nativeBuildInputs = [
    pkgs.openssl
    pkgs.pkg-config
  ];

  # Required so that openssl-sys can be found by pkgconfig.
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

  checkPhase = ''
    cargo test -- --skip "endpoint"
  '';

  dontInstallDocs = true;
  dontInstallManpages = true;
  doInstallCargoArtifacts = false;

  meta = {
    description = "File converters for the cat+ system.";
    homepage = "https://github.com/sdsc-ordes/catplus-converters";
    license = lib.licenses.mit;
    maintainers = [
      "vancauwe"
      "cmdoret"
    ];
  };
}
