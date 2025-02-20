use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1080111604: FileType = FileType {
    file_format: &FileFormat {
        id: 1_080_111_604,
        source_type: SourceType::Iana,
        name: "vnd.otps.ct-kip+xml",
        extensions: &[],
        media_types: &["application/vnd.otps.ct-kip+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
