use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28770292: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_292,
        source_type: SourceType::Wikidata,
        name: "LZ4",
        extensions: &["lz4"],
        media_types: &["application/x-lz4"],
        signatures: &[],
        related_formats: &[],
    },
};
