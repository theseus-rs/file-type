use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28731046: FileType = FileType {
    file_format: &FileFormat {
        id: 28_731_046,
        source_type: SourceType::Wikidata,
        name: "APL Transfer File format",
        extensions: &["atf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
