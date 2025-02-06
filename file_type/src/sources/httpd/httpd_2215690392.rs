use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2215690392: FileFormat = FileFormat {
    id: 2_215_690_392,
    source_type: SourceType::Httpd,
    name: "widget",
    extensions: &["wgt"],
    media_types: &["application/widget"],
    signatures: &[],
    related_formats: &[],
};
