use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28770728: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_728,
        source_type: SourceType::Wikidata,
        name: "MAT-file, Level 5, version 6",
        extensions: &["mat"],
        media_types: &["application/x-matlab-data"],
        signatures: &[],
        related_formats: &[],
    },
};
