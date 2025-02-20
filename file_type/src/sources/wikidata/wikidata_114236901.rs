use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114236901: FileType = FileType {
    file_format: &FileFormat {
        id: 114_236_901,
        source_type: SourceType::Wikidata,
        name: "Browse Database format",
        extensions: &["bsc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
