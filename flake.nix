{
  description = "Waveshare ST7789VM rust boilerplate";

  outputs =
    { self, nixpkgs }:
    {
      packages."x86_64-linux".default =
        (import nixpkgs { system = "x86_64-linux"; }).callPackage ./default.nix
          { };
      packages."aarch64-linux".default =
        (import nixpkgs { system = "aarch64-linux"; }).callPackage ./default.nix
          { };
    };
}
