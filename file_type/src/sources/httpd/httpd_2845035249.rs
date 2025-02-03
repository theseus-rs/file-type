use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2845035249: FileFormat = FileFormat {
    id: 2_845_035_249,
    source_type: SourceType::Httpd,
    name: "stuffitx",
    extensions: &["sitx"],
    media_types: &["application/x-stuffitx"],
    internal_signatures: &[],
    related_formats: &[],
};
