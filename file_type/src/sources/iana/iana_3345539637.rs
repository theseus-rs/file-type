use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3345539637: FileFormat = FileFormat {
    id: 3_345_539_637,
    source_type: SourceType::Iana,
    name: "marc",
    extensions: &[],
    media_types: &["application/marc"],
    internal_signatures: &[],
    related_formats: &[],
};
