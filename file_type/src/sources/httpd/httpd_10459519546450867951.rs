use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_10459519546450867951: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "mrsid image",
    extensions: &["sid"],
    media_types: &["image/x-mrsid-image"],
    internal_signatures: &[],
    related_formats: &[],
};
