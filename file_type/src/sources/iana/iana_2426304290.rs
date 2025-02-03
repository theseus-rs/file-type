use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2426304290: FileFormat = FileFormat {
    id: 2_426_304_290,
    source_type: SourceType::Iana,
    name: "vnd.nokia.n-gage.data",
    extensions: &[],
    media_types: &["application/vnd.nokia.n-gage.data"],
    internal_signatures: &[],
    related_formats: &[],
};
