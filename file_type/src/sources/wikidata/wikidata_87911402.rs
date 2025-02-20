use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_87911402: FileType = FileType {
    file_format: &FileFormat {
        id: 87_911_402,
        source_type: SourceType::Wikidata,
        name: "Avery DesignPro Document 5",
        extensions: &["zdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
