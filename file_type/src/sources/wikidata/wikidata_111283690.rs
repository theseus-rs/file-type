use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111283690: FileType = FileType {
    file_format: &FileFormat {
        id: 111_283_690,
        source_type: SourceType::Wikidata,
        name: "Casio FZ-1 voice dump format",
        extensions: &["fzv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
