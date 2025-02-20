use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1332168616: FileType = FileType {
    file_format: &FileFormat {
        id: 1_332_168_616,
        source_type: SourceType::Iana,
        name: "vnd.ms-playready.initiator+xml",
        extensions: &[],
        media_types: &["application/vnd.ms-playready.initiator+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
