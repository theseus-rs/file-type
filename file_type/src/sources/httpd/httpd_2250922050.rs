use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2250922050: FileFormat = FileFormat {
    id: 2_250_922_050,
    source_type: SourceType::Httpd,
    name: "fpx",
    extensions: &["fpx"],
    media_types: &["image/vnd.fpx"],
    internal_signatures: &[],
    related_formats: &[],
};
