use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58237034: FileType = FileType {
    file_format: &FileFormat {
        id: 58_237_034,
        source_type: SourceType::Wikidata,
        name: "Adobe Multiple Master Metrics font file",
        extensions: &["mmm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
