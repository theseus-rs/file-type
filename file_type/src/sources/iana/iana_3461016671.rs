use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3461016671: FileFormat = FileFormat {
    id: 3_461_016_671,
    source_type: SourceType::Iana,
    name: "vnd.hc+json",
    extensions: &[],
    media_types: &["application/vnd.hc+json"],
    internal_signatures: &[],
    related_formats: &[],
};
