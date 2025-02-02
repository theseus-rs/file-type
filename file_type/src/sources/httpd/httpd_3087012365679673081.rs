use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3087012365679673081: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rn realmedia",
    extensions: &["rm"],
    media_types: &["application/vnd.rn-realmedia"],
    internal_signatures: &[],
    related_formats: &[],
};
