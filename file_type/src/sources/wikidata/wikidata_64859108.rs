use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_64859108: FileType = FileType {
    file_format: &FileFormat {
        id: 64_859_108,
        source_type: SourceType::Wikidata,
        name: "Family Tree Maker Backup file format",
        extensions: &["fbk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
