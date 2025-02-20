use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967384: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_384,
        source_type: SourceType::Wikidata,
        name: "SoundFont 2.0",
        extensions: &["sf2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
