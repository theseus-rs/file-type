use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2495353376: FileFormat = FileFormat {
    id: 2_495_353_376,
    source_type: SourceType::Httpd,
    name: "oasis opendocument text master",
    extensions: &["odm"],
    media_types: &["application/vnd.oasis.opendocument.text-master"],
    signatures: &[],
    related_formats: &[],
};
