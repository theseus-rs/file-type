use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26046105: FileType = FileType {
    file_format: &FileFormat {
        id: 26_046_105,
        source_type: SourceType::Wikidata,
        name: "Q26046105",
        extensions: &[],
        media_types: &["video/AV1", "video/webm; codecs=\"av01.*\""],
        signatures: &[],
        related_formats: &[],
    },
};
