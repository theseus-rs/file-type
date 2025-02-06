use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_863547179: FileFormat = FileFormat {
    id: 863_547_179,
    source_type: SourceType::Httpd,
    name: "xml dtd",
    extensions: &["dtd"],
    media_types: &["application/xml-dtd"],
    signatures: &[],
    related_formats: &[],
};
