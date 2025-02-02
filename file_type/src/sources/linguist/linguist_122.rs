use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_122: FileFormat = FileFormat {
    id: 122,
    source_type: SourceType::Linguist,
    name: "GDB",
    extensions: &["gdb", "gdbinit"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
