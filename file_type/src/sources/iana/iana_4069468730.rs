use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4069468730: FileFormat = FileFormat {
    id: 4_069_468_730,
    source_type: SourceType::Iana,
    name: "vnd.oma.bcast.smartcard-trigger+xml",
    extensions: &[],
    media_types: &["application/vnd.oma.bcast.smartcard-trigger+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
