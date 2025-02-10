use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3281459987: FileType = FileType {
    file_format: &FileFormat {
        id: 3_281_459_987,
        source_type: SourceType::Httpd,
        name: "ief",
        extensions: &["ief"],
        media_types: &["image/ief"],
        signatures: &[],
        related_formats: &[],
    },
};
