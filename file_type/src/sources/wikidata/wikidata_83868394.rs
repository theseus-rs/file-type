use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83868394: FileType = FileType {
    file_format: &FileFormat {
        id: 83_868_394,
        source_type: SourceType::Wikidata,
        name: "SOSI, version 8.1",
        extensions: &["sos"],
        media_types: &["text/vnd.sosi"],
        signatures: &[],
        related_formats: &[],
    },
};
