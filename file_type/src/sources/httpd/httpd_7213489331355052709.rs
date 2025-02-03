use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7213489331355052709: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fli",
    extensions: &["fli"],
    media_types: &["video/x-fli"],
    internal_signatures: &[],
    related_formats: &[],
};
