use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123685792: FileType = FileType {
    file_format: &FileFormat {
        id: 123_685_792,
        source_type: SourceType::Wikidata,
        name: "Adobe InDesign Document, version CC 2023",
        extensions: &["ind", "indd", "indt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
