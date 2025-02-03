use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5788543607611052916: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mediastation cdkey",
    extensions: &["cdkey"],
    media_types: &["application/vnd.mediastation.cdkey"],
    internal_signatures: &[],
    related_formats: &[],
};
