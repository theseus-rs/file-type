use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27996244: FileType = FileType {
    file_format: &FileFormat {
        id: 27_996_244,
        source_type: SourceType::Wikidata,
        name: "HyperCard stack",
        extensions: &["pdf", "tif"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
