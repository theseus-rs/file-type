use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1353407983: FileFormat = FileFormat {
    id: 1_353_407_983,
    source_type: SourceType::Iana,
    name: "vnd.wap.wml",
    extensions: &[],
    media_types: &["text/vnd.wap.wml"],
    internal_signatures: &[],
    related_formats: &[],
};
