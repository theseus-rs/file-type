use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28755730: FileType = FileType {
    file_format: &FileFormat {
        id: 28_755_730,
        source_type: SourceType::Wikidata,
        name: "FDB (Legacy Family Tree)",
        extensions: &["fdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
