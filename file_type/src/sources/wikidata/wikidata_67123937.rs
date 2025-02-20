use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67123937: FileType = FileType {
    file_format: &FileFormat {
        id: 67_123_937,
        source_type: SourceType::Wikidata,
        name: "Print Artist business card file format",
        extensions: &["bc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
