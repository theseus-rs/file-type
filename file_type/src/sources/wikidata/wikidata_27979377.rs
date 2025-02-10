use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979377: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_377,
        source_type: SourceType::Wikidata,
        name: "VobSub subtitle",
        extensions: &["sub"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
