use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2539465323: FileFormat = FileFormat {
    id: 2_539_465_323,
    source_type: SourceType::Httpd,
    name: "png",
    extensions: &["png"],
    media_types: &["image/png"],
    internal_signatures: &[],
    related_formats: &[],
};
