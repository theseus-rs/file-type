use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111317640: FileType = FileType {
    file_format: &FileFormat {
        id: 111_317_640,
        source_type: SourceType::Wikidata,
        name: "MFi - i-Melody - Melody Format for i-Mode",
        extensions: &["mld"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
