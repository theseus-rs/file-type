use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1223779917: FileType = FileType {
    file_format: &FileFormat {
        id: 1_223_779_917,
        source_type: SourceType::Httpd,
        name: "smil xml",
        extensions: &["smi", "smil"],
        media_types: &["application/smil+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
