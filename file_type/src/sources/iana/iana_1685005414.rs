use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1685005414: FileFormat = FileFormat {
    id: 1_685_005_414,
    source_type: SourceType::Iana,
    name: "tab-separated-values",
    extensions: &[],
    media_types: &["text/tab-separated-values"],
    signatures: &[],
    related_formats: &[],
};
