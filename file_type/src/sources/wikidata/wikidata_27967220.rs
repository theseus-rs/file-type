use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967220: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_220,
        source_type: SourceType::Wikidata,
        name: "SoundFX module",
        extensions: &["sfx", "sfx2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
