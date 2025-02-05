use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4002364190: FileFormat = FileFormat {
    id: 4_002_364_190,
    source_type: SourceType::Iana,
    name: "e57",
    extensions: &[],
    media_types: &["model/e57"],
    signatures: &[],
    related_formats: &[],
};
