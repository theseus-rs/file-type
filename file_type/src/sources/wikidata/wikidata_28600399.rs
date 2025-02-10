use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28600399: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_399,
        source_type: SourceType::Wikidata,
        name: "Arma PBO",
        extensions: &["pbo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
