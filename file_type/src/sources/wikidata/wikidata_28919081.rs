use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919081: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_081,
        source_type: SourceType::Wikidata,
        name: "Adobe Premiere Batch List",
        extensions: &["pbl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
