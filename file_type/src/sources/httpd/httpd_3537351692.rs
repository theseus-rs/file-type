use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3537351692: FileFormat = FileFormat {
    id: 3_537_351_692,
    source_type: SourceType::Httpd,
    name: "dece mobile",
    extensions: &["uvm", "uvvm"],
    media_types: &["video/vnd.dece.mobile"],
    signatures: &[],
    related_formats: &[],
};
