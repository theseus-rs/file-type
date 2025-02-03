use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2476393985808283697: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "dreamfactory",
    extensions: &["dfac"],
    media_types: &["application/vnd.dreamfactory"],
    internal_signatures: &[],
    related_formats: &[],
};
