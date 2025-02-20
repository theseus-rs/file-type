use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113486977: FileType = FileType {
    file_format: &FileFormat {
        id: 113_486_977,
        source_type: SourceType::Wikidata,
        name: "Persuasion Mac Document 4.0",
        extensions: &["pn4"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
