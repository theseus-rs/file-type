use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1603773142: FileType = FileType {
    file_format: &FileFormat {
        id: 1_603_773_142,
        source_type: SourceType::Httpd,
        name: "tex",
        extensions: &["tex"],
        media_types: &["application/x-tex"],
        signatures: &[],
        related_formats: &[],
    },
};
