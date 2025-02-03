use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4161761819: FileFormat = FileFormat {
    id: 4_161_761_819,
    source_type: SourceType::Iana,
    name: "fdf",
    extensions: &[],
    media_types: &["application/fdf"],
    internal_signatures: &[],
    related_formats: &[],
};
