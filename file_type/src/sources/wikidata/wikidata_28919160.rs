use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28919160: FileType = FileType {
    file_format: &FileFormat {
        id: 28_919_160,
        source_type: SourceType::Wikidata,
        name: "Standard ACIS Binary",
        extensions: &["sab"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
