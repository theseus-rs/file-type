use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3539896745: FileFormat = FileFormat {
    id: 3_539_896_745,
    source_type: SourceType::Iana,
    name: "vnd.oma.dd2+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.dd2+xml"],
    signatures: &[],
    related_formats: &[],
};
