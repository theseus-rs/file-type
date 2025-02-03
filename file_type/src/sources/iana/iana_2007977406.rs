use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2007977406: FileFormat = FileFormat {
    id: 2_007_977_406,
    source_type: SourceType::Iana,
    name: "ibe-pp-data",
    extensions: &[],
    media_types: &["application/ibe-pp-data"],
    internal_signatures: &[],
    related_formats: &[],
};
