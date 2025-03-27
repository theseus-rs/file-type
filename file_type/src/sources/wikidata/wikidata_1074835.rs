use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1074835: FileType = FileType {
    file_format: &FileFormat {
        id: 1_074_835,
        source_type: SourceType::Wikidata,
        name: "SyncML",
        extensions: &[],
        media_types: &["application/vnd.syncml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
