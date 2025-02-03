use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2764975073: FileFormat = FileFormat {
    id: 2_764_975_073,
    source_type: SourceType::Iana,
    name: "vnd.recordare.musicxml+xml",
    extensions: &[],
    media_types: &["application/vnd.recordare.musicxml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
