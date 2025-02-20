use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127691331: FileType = FileType {
    file_format: &FileFormat {
        id: 127_691_331,
        source_type: SourceType::Wikidata,
        name: "Dylan source code file",
        extensions: &["dylan"],
        media_types: &["text/x-dylan"],
        signatures: &[],
        related_formats: &[],
    },
};
