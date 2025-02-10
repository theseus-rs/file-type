use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_89101317: FileType = FileType {
    file_format: &FileFormat {
        id: 89_101_317,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Gold Project 1",
        extensions: &["ban", "cal", "car", "let", "sig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
