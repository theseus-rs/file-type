use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_734953880: FileType = FileType {
    file_format: &FileFormat {
        id: 734_953_880,
        source_type: SourceType::Iana,
        name: "vorbis-config",
        extensions: &[],
        media_types: &["audio/vorbis-config"],
        signatures: &[],
        related_formats: &[],
    },
};
