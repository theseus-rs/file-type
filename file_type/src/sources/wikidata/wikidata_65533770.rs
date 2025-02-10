use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_65533770: FileType = FileType {
    file_format: &FileFormat {
        id: 65_533_770,
        source_type: SourceType::Wikidata,
        name: "Open Recipe Format",
        extensions: &["orf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
