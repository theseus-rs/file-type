use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600479: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_479,
        source_type: SourceType::Wikidata,
        name: "DOTX",
        extensions: &["dotx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
