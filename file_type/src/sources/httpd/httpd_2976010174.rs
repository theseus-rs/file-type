use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2976010174: FileType = FileType {
    file_format: &FileFormat {
        id: 2_976_010_174,
        source_type: SourceType::Httpd,
        name: "mswrite",
        extensions: &["wri"],
        media_types: &["application/x-mswrite"],
        signatures: &[],
        related_formats: &[],
    },
};
