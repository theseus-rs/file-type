use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4142369363: FileType = FileType {
    file_format: &FileFormat {
        id: 4_142_369_363,
        source_type: SourceType::Iana,
        name: "MP4A-LATM",
        extensions: &[],
        media_types: &["audio/MP4A-LATM"],
        signatures: &[],
        related_formats: &[],
    },
};
