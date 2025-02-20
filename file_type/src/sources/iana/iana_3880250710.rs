use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3880250710: FileType = FileType {
    file_format: &FileFormat {
        id: 3_880_250_710,
        source_type: SourceType::Iana,
        name: "vnd.shade-save-file",
        extensions: &[],
        media_types: &["application/vnd.shade-save-file"],
        signatures: &[],
        related_formats: &[],
    },
};
