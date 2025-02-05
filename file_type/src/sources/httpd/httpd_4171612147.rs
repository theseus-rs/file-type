use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_4171612147: FileFormat = FileFormat {
    id: 4_171_612_147,
    source_type: SourceType::Httpd,
    name: "sun xml calc",
    extensions: &["sxc"],
    media_types: &["application/vnd.sun.xml.calc"],
    signatures: &[],
    related_formats: &[],
};
