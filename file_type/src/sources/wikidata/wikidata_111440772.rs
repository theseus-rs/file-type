use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111440772: FileType = FileType {
    file_format: &FileFormat {
        id: 111_440_772,
        source_type: SourceType::Wikidata,
        name: "Ruby Script",
        extensions: &["rbw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
