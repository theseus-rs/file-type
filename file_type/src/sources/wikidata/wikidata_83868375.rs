use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83868375: FileType = FileType {
    file_format: &FileFormat {
        id: 83_868_375,
        source_type: SourceType::Wikidata,
        name: "SOSI, version 4.1",
        extensions: &["sos"],
        media_types: &["text/vnd.sosi"],
        signatures: &[],
        related_formats: &[],
    },
};
