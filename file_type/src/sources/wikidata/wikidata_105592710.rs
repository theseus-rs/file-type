use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105592710: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_710,
        source_type: SourceType::Wikidata,
        name: "Uncompressed MuseScore format",
        extensions: &["mscx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
