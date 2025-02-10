use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3368114516: FileType = FileType {
    file_format: &FileFormat {
        id: 3_368_114_516,
        source_type: SourceType::Httpd,
        name: "yamaha openscoreformat",
        extensions: &["osf"],
        media_types: &["application/vnd.yamaha.openscoreformat"],
        signatures: &[],
        related_formats: &[],
    },
};
