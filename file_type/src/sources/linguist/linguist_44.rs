use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_44: FileFormat = FileFormat {
    id: 44,
    source_type: SourceType::Linguist,
    name: "C-ObjDump",
    extensions: &["c-objdump"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
