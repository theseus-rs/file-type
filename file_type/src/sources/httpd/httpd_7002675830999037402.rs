use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7002675830999037402: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "hp pcl",
    extensions: &["pcl"],
    media_types: &["application/vnd.hp-pcl"],
    internal_signatures: &[],
    related_formats: &[],
};
