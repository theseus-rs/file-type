use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3398646579: FileFormat = FileFormat {
    id: 3_398_646_579,
    source_type: SourceType::Httpd,
    name: "lzh compressed",
    extensions: &["lzh", "lha"],
    media_types: &["application/x-lzh-compressed"],
    internal_signatures: &[],
    related_formats: &[],
};
