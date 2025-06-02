use bitmask_enum::bitmask;

// TODO: Remove bitmask_enum dependency, as seen below it's not a well match for us

#[bitmask(u16)]
#[derive(Default)]
pub enum AccessFlags {
    Public = 0x0001,
    Private = 0x0002,
    Protected = 0x0004,
    Static = 0x0008,
    Final = 0x0010,
    Synchronized = 0x0020,
    Bridge = 0x0040,
    Varargs = 0x0080,
    Native = 0x0100,
    Abstract = 0x0400,
    Strict = 0x0800,
    Synthetic = 0x1000,
}

impl std::fmt::Display for AccessFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const TO_STRING: [(AccessFlags, &str); 12] = [
            (AccessFlags::Public, "public"),
            (AccessFlags::Private, "private"),
            (AccessFlags::Protected, "protected"),
            (AccessFlags::Static, "static"),
            (AccessFlags::Final, "final"),
            (AccessFlags::Synchronized, "synchronized"),
            (AccessFlags::Bridge, "bridge"),
            (AccessFlags::Varargs, "varargs"),
            (AccessFlags::Native, "native"),
            (AccessFlags::Abstract, "abstract"),
            (AccessFlags::Strict, "strict"),
            (AccessFlags::Synthetic, "synthetic"),
        ];

        for (flag, txt) in TO_STRING {
            if self.intersects(flag) {
                write!(f, "{txt} ")?;
            }
        }

        Ok(())
    }
}
