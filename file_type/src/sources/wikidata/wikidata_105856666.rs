use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105856666: FileType = FileType {
    file_format: &FileFormat {
        id: 105_856_666,
        source_type: SourceType::Wikidata,
        name: "Unreal Engine Project",
        extensions: &["uproject"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
