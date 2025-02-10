use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125523900: FileType = FileType {
    file_format: &FileFormat {
        id: 125_523_900,
        source_type: SourceType::Wikidata,
        name: "Olympus RAW Detail Info file",
        extensions: &["ori"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
