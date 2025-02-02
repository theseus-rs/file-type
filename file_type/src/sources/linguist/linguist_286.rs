use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_286: FileFormat = FileFormat {
    id: 286,
    source_type: SourceType::Linguist,
    name: "PigLatin",
    extensions: &["pig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
