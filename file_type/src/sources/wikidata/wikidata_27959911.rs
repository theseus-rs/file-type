use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27959911: FileType = FileType {
    file_format: &FileFormat {
        id: 27_959_911,
        source_type: SourceType::Wikidata,
        name: "ATRAC Advanced Lossless",
        extensions: &["aa3", "aal", "at3", "oma", "omg"],
        media_types: &["audio/ATRAC-ADVANCED-LOSSLESS"],
        signatures: &[],
        related_formats: &[],
    },
};
