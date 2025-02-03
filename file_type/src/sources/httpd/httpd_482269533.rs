use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_482269533: FileFormat = FileFormat {
    id: 482_269_533,
    source_type: SourceType::Httpd,
    name: "uoml xml",
    extensions: &["uoml"],
    media_types: &["application/vnd.uoml+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
