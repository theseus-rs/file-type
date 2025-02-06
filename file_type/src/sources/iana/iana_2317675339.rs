use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2317675339: FileFormat = FileFormat {
    id: 2_317_675_339,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.chart-template",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.chart-template"],
    signatures: &[],
    related_formats: &[],
};
