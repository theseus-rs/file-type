use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919159: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_159,
        source_type: SourceType::Wikidata,
        name: "Standard ACIS Text",
        extensions: &["sat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
