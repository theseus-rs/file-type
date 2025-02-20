use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_4545411: FileType = FileType {
    file_format: &FileFormat {
        id: 4_545_411,
        source_type: SourceType::Wikidata,
        name: "Blizzard Game Picture",
        extensions: &["blp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
