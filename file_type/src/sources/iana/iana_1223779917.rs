use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1223779917: FileFormat = FileFormat {
    id: 1_223_779_917,
    source_type: SourceType::Iana,
    name: "smil+xml",
    extensions: &[],
    media_types: &["application/smil+xml"],
    signatures: &[],
    related_formats: &[],
};
