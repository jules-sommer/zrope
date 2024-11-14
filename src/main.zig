const std = @import("std");
const tests = @import("test.zig");
pub const Rope = @import("rope.zig").Rope;
pub const Chunks = @import("rope.zig").Chunks;

test {
    std.testing.refAllDecls(Rope);
    std.testing.refAllDecls(Chunks);
    std.testing.refAllDeclsRecursive(tests);
}
