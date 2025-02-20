use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114237297: FileType = FileType {
    file_format: &FileFormat {
        id: 114_237_297,
        source_type: SourceType::Wikidata,
        name: "Visual C++ Project file",
        extensions: &["mak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
