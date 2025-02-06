use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3118572384: FileFormat = FileFormat {
    id: 3_118_572_384,
    source_type: SourceType::Httpd,
    name: "sun xml writer",
    extensions: &["sxw"],
    media_types: &["application/vnd.sun.xml.writer"],
    signatures: &[],
    related_formats: &[],
};
