use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120042266: FileType = FileType {
    file_format: &FileFormat {
        id: 120_042_266,
        source_type: SourceType::Wikidata,
        name: "Cheyenne Backup Script",
        extensions: &["asx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
