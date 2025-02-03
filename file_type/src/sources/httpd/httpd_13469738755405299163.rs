use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13469738755405299163: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "cups ppd",
    extensions: &["ppd"],
    media_types: &["application/vnd.cups-ppd"],
    internal_signatures: &[],
    related_formats: &[],
};
