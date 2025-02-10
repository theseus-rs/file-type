use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_967056: FileType = FileType {
    file_format: &FileFormat {
        id: 967_056,
        source_type: SourceType::Wikidata,
        name: "SoundFont",
        extensions: &["sf2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
