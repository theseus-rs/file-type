use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1690481899: FileFormat = FileFormat {
    id: 1_690_481_899,
    source_type: SourceType::Iana,
    name: "vnd.shootproof+json",
    extensions: &[],
    media_types: &["application/vnd.shootproof+json"],
    internal_signatures: &[],
    related_formats: &[],
};
