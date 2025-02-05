use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3905661247: FileFormat = FileFormat {
    id: 3_905_661_247,
    source_type: SourceType::Httpd,
    name: "oasis opendocument graphics",
    extensions: &["odg"],
    media_types: &["application/vnd.oasis.opendocument.graphics"],
    signatures: &[],
    related_formats: &[],
};
