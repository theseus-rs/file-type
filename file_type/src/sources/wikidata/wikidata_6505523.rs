use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_6505523: FileType = FileType {
    file_format: &FileFormat {
        id: 6_505_523,
        source_type: SourceType::Wikidata,
        name: "Layered Image File Format",
        extensions: &["liff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
