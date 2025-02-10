use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_887473226: FileType = FileType {
    file_format: &FileFormat {
        id: 887_473_226,
        source_type: SourceType::Iana,
        name: "vorbis",
        extensions: &[],
        media_types: &["audio/vorbis"],
        signatures: &[],
        related_formats: &[],
    },
};
