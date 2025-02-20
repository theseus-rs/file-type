use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3216745495: FileType = FileType {
    file_format: &FileFormat {
        id: 3_216_745_495,
        source_type: SourceType::Iana,
        name: "vnd.audiokoz",
        extensions: &[],
        media_types: &["audio/vnd.audiokoz"],
        signatures: &[],
        related_formats: &[],
    },
};
