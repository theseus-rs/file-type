use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_205: FileFormat = FileFormat {
    id: 205,
    source_type: SourceType::Linguist,
    name: "Literate Agda",
    extensions: &["lagda"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
