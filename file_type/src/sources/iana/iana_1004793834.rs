use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1004793834: FileType = FileType {
    file_format: &FileFormat {
        id: 1_004_793_834,
        source_type: SourceType::Iana,
        name: "aac",
        extensions: &[],
        media_types: &["audio/aac"],
        signatures: &[],
        related_formats: &[],
    },
};
