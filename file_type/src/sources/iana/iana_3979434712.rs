use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3979434712: FileFormat = FileFormat {
    id: 3_979_434_712,
    source_type: SourceType::Iana,
    name: "heif",
    extensions: &[],
    media_types: &["image/heif"],
    internal_signatures: &[],
    related_formats: &[],
};
