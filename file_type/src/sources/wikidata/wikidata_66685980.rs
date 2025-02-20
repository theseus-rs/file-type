use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66685980: FileType = FileType {
    file_format: &FileFormat {
        id: 66_685_980,
        source_type: SourceType::Wikidata,
        name: "OR2",
        extensions: &["or2"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
