use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_285: FileFormat = FileFormat {
    id: 285,
    source_type: SourceType::Linguist,
    name: "PicoLisp",
    extensions: &["l"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
