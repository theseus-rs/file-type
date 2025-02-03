use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2904866414: FileFormat = FileFormat {
    id: 2_904_866_414,
    source_type: SourceType::Httpd,
    name: "jisp",
    extensions: &["jisp"],
    media_types: &["application/vnd.jisp"],
    internal_signatures: &[],
    related_formats: &[],
};
