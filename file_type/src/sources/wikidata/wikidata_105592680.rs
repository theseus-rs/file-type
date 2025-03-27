use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105592680: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_680,
        source_type: SourceType::Wikidata,
        name: "Guitar Pro 6 tablature",
        extensions: &["gpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
