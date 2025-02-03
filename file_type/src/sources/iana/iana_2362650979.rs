use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2362650979: FileFormat = FileFormat {
    id: 2_362_650_979,
    source_type: SourceType::Iana,
    name: "CN",
    extensions: &[],
    media_types: &["audio/CN"],
    internal_signatures: &[],
    related_formats: &[],
};
