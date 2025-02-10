use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_122075846: FileType = FileType {
    file_format: &FileFormat {
        id: 122_075_846,
        source_type: SourceType::Wikidata,
        name: "Finale Lesson File",
        extensions: &["lsn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
