use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1929696087: FileFormat = FileFormat {
    id: 1_929_696_087,
    source_type: SourceType::Iana,
    name: "vnd.uri-map",
    extensions: &[],
    media_types: &["application/vnd.uri-map"],
    internal_signatures: &[],
    related_formats: &[],
};
