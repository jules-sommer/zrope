inputs:
with inputs; [
  (_: prev: {
    neovim = neovim.packages.${prev.system}.default;
    zig = zig.packages.${prev.system}.nightly;
    inherit (zls.packages.${prev.system}) zls;
  })
]
