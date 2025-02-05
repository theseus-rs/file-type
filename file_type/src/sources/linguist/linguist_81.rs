use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_81: FileFormat = FileFormat {
    id: 81,
    source_type: SourceType::Linguist,
    name: "D-ObjDump",
    extensions: &["d-objdump"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
