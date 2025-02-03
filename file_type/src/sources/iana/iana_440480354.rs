use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_440480354: FileFormat = FileFormat {
    id: 440_480_354,
    source_type: SourceType::Iana,
    name: "vnd.3gpp.v2x",
    extensions: &[],
    media_types: &["application/vnd.3gpp.v2x"],
    internal_signatures: &[],
    related_formats: &[],
};
