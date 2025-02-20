use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128583427: FileType = FileType {
    file_format: &FileFormat {
        id: 128_583_427,
        source_type: SourceType::Wikidata,
        name: "ABAP file format",
        extensions: &["abap"],
        media_types: &["text/x-abap"],
        signatures: &[],
        related_formats: &[],
    },
};
