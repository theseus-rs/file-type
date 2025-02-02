use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3184275637844130277: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "framemaker",
    extensions: &["fm", "frame", "maker", "book"],
    media_types: &["application/vnd.framemaker"],
    internal_signatures: &[],
    related_formats: &[],
};
