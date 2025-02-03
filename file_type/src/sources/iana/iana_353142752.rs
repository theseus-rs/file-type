use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_353142752: FileFormat = FileFormat {
    id: 353_142_752,
    source_type: SourceType::Iana,
    name: "media-policy-dataset+xml",
    extensions: &[],
    media_types: &["application/media-policy-dataset+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
