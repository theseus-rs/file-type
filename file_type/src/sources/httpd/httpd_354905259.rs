use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_354905259: FileType = FileType {
    file_format: &FileFormat {
        id: 354_905_259,
        source_type: SourceType::Httpd,
        name: "sbml xml",
        extensions: &["sbml"],
        media_types: &["application/sbml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
