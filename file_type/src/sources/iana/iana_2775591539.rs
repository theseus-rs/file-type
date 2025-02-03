use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2775591539: FileFormat = FileFormat {
    id: 2_775_591_539,
    source_type: SourceType::Iana,
    name: "vnd.ecdis-update",
    extensions: &[],
    media_types: &["application/vnd.ecdis-update"],
    internal_signatures: &[],
    related_formats: &[],
};
