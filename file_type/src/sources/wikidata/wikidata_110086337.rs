use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110086337: FileType = FileType {
    file_format: &FileFormat {
        id: 110_086_337,
        source_type: SourceType::Wikidata,
        name: "Cool Edit/Adobe Audition Session File (Binary)",
        extensions: &["ses"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
