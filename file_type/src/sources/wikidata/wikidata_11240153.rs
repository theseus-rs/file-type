use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_11240153: FileType = FileType {
    file_format: &FileFormat {
        id: 11_240_153,
        source_type: SourceType::Wikidata,
        name: "Q0 Bitmap",
        extensions: &["q0", "rgb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
