use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1079094050: FileFormat = FileFormat {
    id: 1_079_094_050,
    source_type: SourceType::Iana,
    name: "hsj2 (OBSOLETE)",
    extensions: &[],
    media_types: &["image/hsj2"],
    signatures: &[],
    related_formats: &[],
};
