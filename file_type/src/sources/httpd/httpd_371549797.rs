use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_371549797: FileFormat = FileFormat {
    id: 371_549_797,
    source_type: SourceType::Httpd,
    name: "dece video",
    extensions: &["uvv", "uvvv"],
    media_types: &["video/vnd.dece.video"],
    internal_signatures: &[],
    related_formats: &[],
};
