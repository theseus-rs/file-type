use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_272413369: FileType = FileType {
    file_format: &FileFormat {
        id: 272_413_369,
        source_type: SourceType::Iana,
        name: "RED",
        extensions: &[],
        media_types: &["audio/RED"],
        signatures: &[],
        related_formats: &[],
    },
};
