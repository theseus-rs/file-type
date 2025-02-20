use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27967351: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_351,
        source_type: SourceType::Wikidata,
        name: "iTunes Music Library, XML variant",
        extensions: &["xml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
