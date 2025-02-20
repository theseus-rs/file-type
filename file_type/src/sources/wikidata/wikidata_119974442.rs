use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119974442: FileType = FileType {
    file_format: &FileFormat {
        id: 119_974_442,
        source_type: SourceType::Wikidata,
        name: "WebEasy Template",
        extensions: &["tpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
