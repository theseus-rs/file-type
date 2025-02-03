use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2167490047: FileFormat = FileFormat {
    id: 2_167_490_047,
    source_type: SourceType::Iana,
    name: "apng",
    extensions: &[],
    media_types: &["image/apng"],
    internal_signatures: &[],
    related_formats: &[],
};
