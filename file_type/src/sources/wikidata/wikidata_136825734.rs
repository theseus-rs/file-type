use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136825734: FileType = FileType {
    file_format: &FileFormat {
        id: 136_825_734,
        source_type: SourceType::Wikidata,
        name: "Freeway file",
        extensions: &["freeway"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
