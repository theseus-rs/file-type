use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_120867991: FileType = FileType {
    file_format: &FileFormat {
        id: 120_867_991,
        source_type: SourceType::Wikidata,
        name: "Cumulus Backup File",
        extensions: &["bak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
