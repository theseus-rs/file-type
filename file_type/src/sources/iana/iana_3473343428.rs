use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3473343428: FileFormat = FileFormat {
    id: 3_473_343_428,
    source_type: SourceType::Iana,
    name: "naplps",
    extensions: &[],
    media_types: &["image/naplps"],
    internal_signatures: &[],
    related_formats: &[],
};
