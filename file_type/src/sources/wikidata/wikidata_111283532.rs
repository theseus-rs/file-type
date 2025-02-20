use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111283532: FileType = FileType {
    file_format: &FileFormat {
        id: 111_283_532,
        source_type: SourceType::Wikidata,
        name: "Casio FZ-1 bank dump format",
        extensions: &["fzb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
