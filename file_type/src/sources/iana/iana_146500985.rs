use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_146500985: FileFormat = FileFormat {
    id: 146_500_985,
    source_type: SourceType::Iana,
    name: "vnd.psfs",
    extensions: &[],
    media_types: &["application/vnd.psfs"],
    internal_signatures: &[],
    related_formats: &[],
};
