use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_86996065: FileType = FileType {
    file_format: &FileFormat {
        id: 86_996_065,
        source_type: SourceType::Wikidata,
        name: "PaperPort MAX 5-7",
        extensions: &["max"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
