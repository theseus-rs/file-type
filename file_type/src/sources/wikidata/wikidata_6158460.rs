use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_6158460: FileType = FileType {
    file_format: &FileFormat {
        id: 6_158_460,
        source_type: SourceType::Wikidata,
        name: "Video Recording Object file",
        extensions: &["vro"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
