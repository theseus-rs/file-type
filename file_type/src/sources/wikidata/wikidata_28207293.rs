use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207293: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_293,
        source_type: SourceType::Wikidata,
        name: "Softimage PIC",
        extensions: &["pic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
