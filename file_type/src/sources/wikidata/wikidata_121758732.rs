use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_121758732: FileType = FileType {
    file_format: &FileFormat {
        id: 121_758_732,
        source_type: SourceType::Wikidata,
        name: "Family Tree Maker FTMB Backup File",
        extensions: &["ftmb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
