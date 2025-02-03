use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2061607266: FileFormat = FileFormat {
    id: 2_061_607_266,
    source_type: SourceType::Iana,
    name: "geopose+json",
    extensions: &[],
    media_types: &["application/geopose+json"],
    internal_signatures: &[],
    related_formats: &[],
};
