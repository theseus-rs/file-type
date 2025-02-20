use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757880: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_880,
        source_type: SourceType::Wikidata,
        name: "git packfile",
        extensions: &["pack"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
