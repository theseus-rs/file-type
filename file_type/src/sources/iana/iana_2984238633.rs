use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2984238633: FileFormat = FileFormat {
    id: 2_984_238_633,
    source_type: SourceType::Iana,
    name: "x3d+xml",
    extensions: &[],
    media_types: &["model/x3d+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
