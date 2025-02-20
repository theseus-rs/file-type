use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_86996249: FileType = FileType {
    file_format: &FileFormat {
        id: 86_996_249,
        source_type: SourceType::Wikidata,
        name: "PaperPort MAX 3-4",
        extensions: &["max"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
