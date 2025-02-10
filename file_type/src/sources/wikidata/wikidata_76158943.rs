use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_76158943: FileType = FileType {
    file_format: &FileFormat {
        id: 76_158_943,
        source_type: SourceType::Wikidata,
        name: "MegaPaint VPO",
        extensions: &["vpo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
