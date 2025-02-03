use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_6415018873001411960: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "apple diskimage",
    extensions: &["dmg"],
    media_types: &["application/x-apple-diskimage"],
    internal_signatures: &[],
    related_formats: &[],
};
