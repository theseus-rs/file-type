use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1013589051: FileFormat = FileFormat {
    id: 1_013_589_051,
    source_type: SourceType::Iana,
    name: "wordperfect5.1",
    extensions: &[],
    media_types: &["application/wordperfect5.1"],
    internal_signatures: &[],
    related_formats: &[],
};
