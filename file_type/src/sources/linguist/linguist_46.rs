use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_46: FileFormat = FileFormat {
    id: 46,
    source_type: SourceType::Linguist,
    name: "CLIPS",
    extensions: &["clp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
