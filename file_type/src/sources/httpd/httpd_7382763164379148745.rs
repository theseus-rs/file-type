use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7382763164379148745: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "jpm",
    extensions: &["jpm", "jpgm"],
    media_types: &["video/jpm"],
    internal_signatures: &[],
    related_formats: &[],
};
