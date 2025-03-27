use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_53756539: FileType = FileType {
    file_format: &FileFormat {
        id: 53_756_539,
        source_type: SourceType::Wikidata,
        name: "git packfile index, version 1",
        extensions: &["idx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
