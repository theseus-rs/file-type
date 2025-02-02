use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_27: FileFormat = FileFormat {
    id: 27,
    source_type: SourceType::Linguist,
    name: "AutoIt",
    extensions: &["au3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
