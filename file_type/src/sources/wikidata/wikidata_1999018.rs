use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1999018: FileType = FileType {
    file_format: &FileFormat {
        id: 1_999_018,
        source_type: SourceType::Wikidata,
        name: "VC-1",
        extensions: &[],
        media_types: &["video/vc1"],
        signatures: &[],
        related_formats: &[],
    },
};
