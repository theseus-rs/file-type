use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_56827160: FileType = FileType {
    file_format: &FileFormat {
        id: 56_827_160,
        source_type: SourceType::Wikidata,
        name: "GPS eXchange Format, version 1.0",
        extensions: &["gpx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
