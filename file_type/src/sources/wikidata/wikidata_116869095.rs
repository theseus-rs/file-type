use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116869095: FileType = FileType {
    file_format: &FileFormat {
        id: 116_869_095,
        source_type: SourceType::Wikidata,
        name: "Summitsoft Letterhead",
        extensions: &["lth"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
