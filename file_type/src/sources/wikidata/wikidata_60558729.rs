use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60558729: FileType = FileType {
    file_format: &FileFormat {
        id: 60_558_729,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Painting, version 2",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
