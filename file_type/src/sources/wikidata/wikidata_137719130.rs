use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137719130: FileType = FileType {
    file_format: &FileFormat {
        id: 137_719_130,
        source_type: SourceType::Wikidata,
        name: "Calligra Plan Document",
        extensions: &["plan"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
