use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_636189500: FileFormat = FileFormat {
    id: 636_189_500,
    source_type: SourceType::Iana,
    name: "vnd.apothekende.reservation+json",
    extensions: &[],
    media_types: &["application/vnd.apothekende.reservation+json"],
    signatures: &[],
    related_formats: &[],
};
