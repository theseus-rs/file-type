use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3315827232: FileFormat = FileFormat {
    id: 3_315_827_232,
    source_type: SourceType::Httpd,
    name: "oasis opendocument text template",
    extensions: &["ott"],
    media_types: &["application/vnd.oasis.opendocument.text-template"],
    internal_signatures: &[],
    related_formats: &[],
};
