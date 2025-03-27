use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_53756551: FileType = FileType {
    file_format: &FileFormat {
        id: 53_756_551,
        source_type: SourceType::Wikidata,
        name: "git packfile index, version 2",
        extensions: &["idx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
