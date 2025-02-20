use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123483270: FileType = FileType {
    file_format: &FileFormat {
        id: 123_483_270,
        source_type: SourceType::Wikidata,
        name: "executable Python zip archive (.pyz)",
        extensions: &["pyz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
