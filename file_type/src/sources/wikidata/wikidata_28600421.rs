use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600421: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_421,
        source_type: SourceType::Wikidata,
        name: "Ant build file",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
