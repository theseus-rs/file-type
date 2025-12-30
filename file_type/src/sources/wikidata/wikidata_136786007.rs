use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136786007: FileType = FileType {
    file_format: &FileFormat {
        id: 136_786_007,
        source_type: SourceType::Wikidata,
        name: "Windows Sticky Note file",
        extensions: &["snt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
