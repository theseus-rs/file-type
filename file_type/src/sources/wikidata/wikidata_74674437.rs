use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_74674437: FileType = FileType {
    file_format: &FileFormat {
        id: 74_674_437,
        source_type: SourceType::Wikidata,
        name: "Kindle app book info",
        extensions: &["ticr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
