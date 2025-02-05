use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2284811522: FileFormat = FileFormat {
    id: 2_284_811_522,
    source_type: SourceType::Iana,
    name: "spdx+json",
    extensions: &[],
    media_types: &["application/spdx+json"],
    signatures: &[],
    related_formats: &[],
};
