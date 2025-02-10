use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_25099931: FileType = FileType {
    file_format: &FileFormat {
        id: 25_099_931,
        source_type: SourceType::Wikidata,
        name: "Scratch Project SB2",
        extensions: &["sb2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
