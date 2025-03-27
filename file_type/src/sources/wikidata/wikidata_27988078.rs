use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27988078: FileType = FileType {
    file_format: &FileFormat {
        id: 27_988_078,
        source_type: SourceType::Wikidata,
        name: "ST disk image",
        extensions: &["st"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
