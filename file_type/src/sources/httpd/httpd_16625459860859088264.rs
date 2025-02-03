use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16625459860859088264: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oasis opendocument text master",
    extensions: &["odm"],
    media_types: &["application/vnd.oasis.opendocument.text-master"],
    internal_signatures: &[],
    related_formats: &[],
};
