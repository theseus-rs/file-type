use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105592678: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_678,
        source_type: SourceType::Wikidata,
        name: "Guitar Pro 3 tablature",
        extensions: &["gp3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
