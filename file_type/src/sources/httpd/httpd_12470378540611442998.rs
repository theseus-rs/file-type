use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_12470378540611442998: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "3ds",
    extensions: &["3ds"],
    media_types: &["image/x-3ds"],
    internal_signatures: &[],
    related_formats: &[],
};
