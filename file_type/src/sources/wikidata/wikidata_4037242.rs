use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4037242: FileType = FileType {
    file_format: &FileFormat {
        id: 4_037_242,
        source_type: SourceType::Wikidata,
        name: "Desktop.ini",
        extensions: &["ini"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
