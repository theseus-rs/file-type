use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8603435392360225765: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "xiff",
    extensions: &["xif"],
    media_types: &["image/vnd.xiff"],
    internal_signatures: &[],
    related_formats: &[],
};
