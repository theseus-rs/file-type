use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600473: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_473,
        source_type: SourceType::Wikidata,
        name: "DMK",
        extensions: &["dmk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
