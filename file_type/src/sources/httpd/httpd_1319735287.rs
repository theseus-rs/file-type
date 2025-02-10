use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1319735287: FileType = FileType {
    file_format: &FileFormat {
        id: 1_319_735_287,
        source_type: SourceType::Httpd,
        name: "sparql results xml",
        extensions: &["srx"],
        media_types: &["application/sparql-results+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
