use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_7212275662721677375: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "oasis opendocument image",
    extensions: &["odi"],
    media_types: &["application/vnd.oasis.opendocument.image"],
    internal_signatures: &[],
    related_formats: &[],
};
