use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113486673: FileType = FileType {
    file_format: &FileFormat {
        id: 113_486_673,
        source_type: SourceType::Wikidata,
        name: "Persuasion Mac Document 2.1",
        extensions: &["pr2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
