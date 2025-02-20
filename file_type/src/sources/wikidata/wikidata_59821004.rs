use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_59821004: FileType = FileType {
    file_format: &FileFormat {
        id: 59_821_004,
        source_type: SourceType::Wikidata,
        name: "Exchangeable Image File Format (Audio)",
        extensions: &["wav"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
