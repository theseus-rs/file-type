use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28344013: FileType = FileType {
    file_format: &FileFormat {
        id: 28_344_013,
        source_type: SourceType::Wikidata,
        name: "BACKUP",
        extensions: &["@@@"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
