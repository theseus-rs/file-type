use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1670429068: FileFormat = FileFormat {
    id: 1_670_429_068,
    source_type: SourceType::Iana,
    name: "iges",
    extensions: &[],
    media_types: &["model/iges"],
    internal_signatures: &[],
    related_formats: &[],
};
