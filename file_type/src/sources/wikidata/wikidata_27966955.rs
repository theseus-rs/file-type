use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27966955: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_955,
        source_type: SourceType::Wikidata,
        name: "USF",
        extensions: &["miniusf", "usf", "usflib"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
