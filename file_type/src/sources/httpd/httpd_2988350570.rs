use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2988350570: FileType = FileType {
    file_format: &FileFormat {
        id: 2_988_350_570,
        source_type: SourceType::Httpd,
        name: "font ghostscript",
        extensions: &["gsf"],
        media_types: &["application/x-font-ghostscript"],
        signatures: &[],
        related_formats: &[],
    },
};
