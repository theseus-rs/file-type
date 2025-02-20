use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1220007935: FileType = FileType {
    file_format: &FileFormat {
        id: 1_220_007_935,
        source_type: SourceType::Httpd,
        name: "richtext",
        extensions: &["rtx"],
        media_types: &["text/richtext"],
        signatures: &[],
        related_formats: &[],
    },
};
