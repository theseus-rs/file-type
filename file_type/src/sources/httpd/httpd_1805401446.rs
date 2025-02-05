use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1805401446: FileFormat = FileFormat {
    id: 1_805_401_446,
    source_type: SourceType::Httpd,
    name: "authorware map",
    extensions: &["aam"],
    media_types: &["application/x-authorware-map"],
    signatures: &[],
    related_formats: &[],
};
