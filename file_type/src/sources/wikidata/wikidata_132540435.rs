use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132540435: FileType = FileType {
    file_format: &FileFormat {
        id: 132_540_435,
        source_type: SourceType::Wikidata,
        name: "Well Properties file format",
        extensions: &["wprops"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
