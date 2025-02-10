use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_88387779: FileType = FileType {
    file_format: &FileFormat {
        id: 88_387_779,
        source_type: SourceType::Wikidata,
        name: "FamilyTree Maker Database 5-16",
        extensions: &["fbk", "ftw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
