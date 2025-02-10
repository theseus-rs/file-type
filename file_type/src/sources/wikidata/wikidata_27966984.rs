use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966984: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_984,
        source_type: SourceType::Wikidata,
        name: "Actionamics Sound Tool",
        extensions: &["ast"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
