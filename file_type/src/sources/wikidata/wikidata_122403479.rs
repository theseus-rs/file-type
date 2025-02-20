use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122403479: FileType = FileType {
    file_format: &FileFormat {
        id: 122_403_479,
        source_type: SourceType::Wikidata,
        name: "CodeWarrior Resource File",
        extensions: &["rsrc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
