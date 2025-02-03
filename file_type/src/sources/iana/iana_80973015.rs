use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_80973015: FileFormat = FileFormat {
    id: 80_973_015,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.presentation",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.presentation"],
    internal_signatures: &[],
    related_formats: &[],
};
