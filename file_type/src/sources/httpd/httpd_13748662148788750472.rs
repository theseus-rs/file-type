use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13748662148788750472: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tar",
    extensions: &["tar"],
    media_types: &["application/x-tar"],
    internal_signatures: &[],
    related_formats: &[],
};
