use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_1308395743: FileType = FileType {
    file_format: &FileFormat {
        id: 1_308_395_743,
        source_type: SourceType::Httpd,
        name: "basic",
        extensions: &["au", "snd"],
        media_types: &["audio/basic"],
        signatures: &[],
        related_formats: &[],
    },
};
