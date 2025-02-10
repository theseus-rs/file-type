use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83868385: FileType = FileType {
    file_format: &FileFormat {
        id: 83_868_385,
        source_type: SourceType::Wikidata,
        name: "SOSI, version 4.5",
        extensions: &["sos"],
        media_types: &["text/vnd.sosi"],
        signatures: &[],
        related_formats: &[],
    },
};
