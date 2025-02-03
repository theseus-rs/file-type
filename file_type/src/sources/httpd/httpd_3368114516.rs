use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3368114516: FileFormat = FileFormat {
    id: 3_368_114_516,
    source_type: SourceType::Httpd,
    name: "yamaha openscoreformat",
    extensions: &["osf"],
    media_types: &["application/vnd.yamaha.openscoreformat"],
    internal_signatures: &[],
    related_formats: &[],
};
