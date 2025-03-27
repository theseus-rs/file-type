use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6043826: FileType = FileType {
    file_format: &FileFormat {
        id: 6_043_826,
        source_type: SourceType::Wikidata,
        name: "Intellifont",
        extensions: &["lib", "type"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
