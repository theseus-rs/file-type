use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_119158915: FileType = FileType {
    file_format: &FileFormat {
        id: 119_158_915,
        source_type: SourceType::Wikidata,
        name: "Digital Image PNG Plus",
        extensions: &["png"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
