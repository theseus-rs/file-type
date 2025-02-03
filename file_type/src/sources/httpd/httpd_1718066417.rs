use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1718066417: FileFormat = FileFormat {
    id: 1_718_066_417,
    source_type: SourceType::Httpd,
    name: "fli",
    extensions: &["fli"],
    media_types: &["video/x-fli"],
    internal_signatures: &[],
    related_formats: &[],
};
