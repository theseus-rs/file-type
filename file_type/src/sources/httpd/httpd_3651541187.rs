use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3651541187: FileFormat = FileFormat {
    id: 3_651_541_187,
    source_type: SourceType::Httpd,
    name: "immervision ivu",
    extensions: &["ivu"],
    media_types: &["application/vnd.immervision-ivu"],
    internal_signatures: &[],
    related_formats: &[],
};
