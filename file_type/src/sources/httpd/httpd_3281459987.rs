use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3281459987: FileFormat = FileFormat {
    id: 3_281_459_987,
    source_type: SourceType::Httpd,
    name: "ief",
    extensions: &["ief"],
    media_types: &["image/ief"],
    internal_signatures: &[],
    related_formats: &[],
};
