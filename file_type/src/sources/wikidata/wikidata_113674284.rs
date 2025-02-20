use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113674284: FileType = FileType {
    file_format: &FileFormat {
        id: 113_674_284,
        source_type: SourceType::Wikidata,
        name: "Final Draft Secure Copy",
        extensions: &["fds"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
