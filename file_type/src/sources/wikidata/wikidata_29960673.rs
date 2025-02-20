use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29960673: FileType = FileType {
    file_format: &FileFormat {
        id: 29_960_673,
        source_type: SourceType::Wikidata,
        name: "Avantes USB spectrometer ROH file",
        extensions: &["roh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
