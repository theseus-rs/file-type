use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_79239537: FileType = FileType {
    file_format: &FileFormat {
        id: 79_239_537,
        source_type: SourceType::Wikidata,
        name: "AOL Address Book",
        extensions: &["aby"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
