const std = @import("std");
const tests = @import("test.zig");
const rope = @import("rope.zig");

pub const Rope = rope.Rope;
pub const Chunks = rope.Chunks;

test {
    std.testing.refAllDecls(Rope);
    std.testing.refAllDecls(Chunks);
    std.testing.refAllDeclsRecursive(tests);
}
