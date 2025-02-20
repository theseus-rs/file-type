use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967426: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_426,
        source_type: SourceType::Wikidata,
        name: "Creative Music System",
        extensions: &["cms"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
