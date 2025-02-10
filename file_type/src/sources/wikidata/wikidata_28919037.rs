use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919037: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_037,
        source_type: SourceType::Wikidata,
        name: "Type-2 DV AVI",
        extensions: &["avi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
