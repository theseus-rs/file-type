use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_100377201: FileType = FileType {
    file_format: &FileFormat {
        id: 100_377_201,
        source_type: SourceType::Wikidata,
        name: "HP TRIM Outlook Saved Message File",
        extensions: &["vmbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
