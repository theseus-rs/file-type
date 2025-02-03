use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_582845968: FileFormat = FileFormat {
    id: 582_845_968,
    source_type: SourceType::Iana,
    name: "H265",
    extensions: &[],
    media_types: &["video/H265"],
    internal_signatures: &[],
    related_formats: &[],
};
