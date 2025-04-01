use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_133575413: FileType = FileType {
    file_format: &FileFormat {
        id: 133_575_413,
        source_type: SourceType::Wikidata,
        name: "Face Painter file",
        extensions: &["fcp", "fpt"],
        media_types: &["image/x-face-painter"],
        signatures: &[],
        related_formats: &[],
    },
};
