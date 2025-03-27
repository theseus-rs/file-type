use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105592714: FileType = FileType {
    file_format: &FileFormat {
        id: 105_592_714,
        source_type: SourceType::Wikidata,
        name: "MuseScore backup file",
        extensions: &["mscx", "mscz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
