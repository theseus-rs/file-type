use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_361957890: FileFormat = FileFormat {
    id: 361_957_890,
    source_type: SourceType::Httpd,
    name: "syncml dm wbxml",
    extensions: &["bdm"],
    media_types: &["application/vnd.syncml.dm+wbxml"],
    signatures: &[],
    related_formats: &[],
};
