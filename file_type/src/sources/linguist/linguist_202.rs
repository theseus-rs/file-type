use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_202: FileFormat = FileFormat {
    id: 202,
    source_type: SourceType::Linguist,
    name: "Linker Script",
    extensions: &["ld", "lds", "x"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
