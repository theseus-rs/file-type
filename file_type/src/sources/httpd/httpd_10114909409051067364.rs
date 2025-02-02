use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10114909409051067364: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sun xml writer",
    extensions: &["sxw"],
    media_types: &["application/vnd.sun.xml.writer"],
    internal_signatures: &[],
    related_formats: &[],
};
