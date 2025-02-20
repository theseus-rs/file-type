use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206349: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_349,
        source_type: SourceType::Wikidata,
        name: "GEOS Convert",
        extensions: &["cvt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
