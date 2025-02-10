use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66685988: FileType = FileType {
    file_format: &FileFormat {
        id: 66_685_988,
        source_type: SourceType::Wikidata,
        name: "OR5",
        extensions: &["or5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
