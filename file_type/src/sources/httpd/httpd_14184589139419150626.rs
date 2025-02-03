use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_14184589139419150626: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "avif",
    extensions: &["avif"],
    media_types: &["image/avif"],
    internal_signatures: &[],
    related_formats: &[],
};
