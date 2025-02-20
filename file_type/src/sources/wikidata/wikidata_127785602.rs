use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_127785602: FileType = FileType {
    file_format: &FileFormat {
        id: 127_785_602,
        source_type: SourceType::Wikidata,
        name: "MetaPost file",
        extensions: &["mp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
