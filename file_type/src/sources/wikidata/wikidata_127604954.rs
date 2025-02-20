use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127604954: FileType = FileType {
    file_format: &FileFormat {
        id: 127_604_954,
        source_type: SourceType::Wikidata,
        name: "Arduino Sketch file",
        extensions: &["ino"],
        media_types: &["text/x-arduino"],
        signatures: &[],
        related_formats: &[],
    },
};
