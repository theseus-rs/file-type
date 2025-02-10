use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27966903: FileType = FileType {
    file_format: &FileFormat {
        id: 27_966_903,
        source_type: SourceType::Wikidata,
        name: "KSSX",
        extensions: &["kss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
