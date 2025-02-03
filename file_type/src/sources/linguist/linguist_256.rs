use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_256: FileFormat = FileFormat {
    id: 256,
    source_type: SourceType::Linguist,
    name: "ObjDump",
    extensions: &["objdump"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
