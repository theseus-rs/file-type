use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28771221: FileType = FileType {
    file_format: &FileFormat {
        id: 28_771_221,
        source_type: SourceType::Wikidata,
        name: "MAT-file, Level 5, version 7.3",
        extensions: &["mat"],
        media_types: &["application/x-matlab-data"],
        signatures: &[],
        related_formats: &[],
    },
};
