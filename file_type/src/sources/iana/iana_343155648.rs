use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_343155648: FileFormat = FileFormat {
    id: 343_155_648,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.sprov+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.sprov+xml"],
    signatures: &[],
    related_formats: &[],
};
