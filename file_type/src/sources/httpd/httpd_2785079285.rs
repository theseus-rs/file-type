use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2785079285: FileType = FileType {
    file_format: &FileFormat {
        id: 2_785_079_285,
        source_type: SourceType::Httpd,
        name: "immervision ivp",
        extensions: &["ivp"],
        media_types: &["application/vnd.immervision-ivp"],
        signatures: &[],
        related_formats: &[],
    },
};
