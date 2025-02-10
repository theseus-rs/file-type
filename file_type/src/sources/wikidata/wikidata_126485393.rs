use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_126485393: FileType = FileType {
    file_format: &FileFormat {
        id: 126_485_393,
        source_type: SourceType::Wikidata,
        name: "Comic Book ACE Archive",
        extensions: &["cba"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
