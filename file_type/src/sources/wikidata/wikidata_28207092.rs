use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207092: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_092,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Index file",
        extensions: &["sdx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
