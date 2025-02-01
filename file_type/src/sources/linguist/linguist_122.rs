use crate::format::FileFormat;

pub(crate) const LINGUIST_122: FileFormat = FileFormat {
    id: 122,
    puid: "linguist/122",
    name: "GDB",
    extensions: &["gdb", "gdbinit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
