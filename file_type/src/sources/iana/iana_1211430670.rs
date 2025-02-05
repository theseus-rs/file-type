use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1211430670: FileFormat = FileFormat {
    id: 1_211_430_670,
    source_type: SourceType::Iana,
    name: "vnd.oasis.opendocument.text",
    extensions: &[],
    media_types: &["application/vnd.oasis.opendocument.text"],
    signatures: &[],
    related_formats: &[],
};
