use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3852266454: FileType = FileType {
    file_format: &FileFormat {
        id: 3_852_266_454,
        source_type: SourceType::Httpd,
        name: "wordperfect",
        extensions: &["wpd"],
        media_types: &["application/vnd.wordperfect"],
        signatures: &[],
        related_formats: &[],
    },
};
