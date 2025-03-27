use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_126485393: FileType = FileType {
    file_format: &FileFormat {
        id: 126_485_393,
        source_type: SourceType::Wikidata,
        name: "Comic Book ACE Archive",
        extensions: &["cba"],
        media_types: &["application/x-cba"],
        signatures: &[],
        related_formats: &[],
    },
};
