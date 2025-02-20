use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_32097899: FileType = FileType {
    file_format: &FileFormat {
        id: 32_097_899,
        source_type: SourceType::Wikidata,
        name: "Fallout v2 DAT",
        extensions: &["dat"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
