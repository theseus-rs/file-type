use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2733296093: FileFormat = FileFormat {
    id: 2_733_296_093,
    source_type: SourceType::Iana,
    name: "vnd.nimn",
    extensions: &[],
    media_types: &["application/vnd.nimn"],
    internal_signatures: &[],
    related_formats: &[],
};
