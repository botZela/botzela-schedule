_default:
  just --list

docker:
  nix build .#dockerImage
  ./result | docker load

deploy:
  fly deploy --local-only -i botzela-schedule-nix
