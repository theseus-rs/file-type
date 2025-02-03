use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_948240628: FileFormat = FileFormat {
    id: 948_240_628,
    source_type: SourceType::Httpd,
    name: "mpeg",
    extensions: &["mpeg", "mpg", "mpe", "m1v", "m2v"],
    media_types: &["video/mpeg"],
    internal_signatures: &[],
    related_formats: &[],
};
