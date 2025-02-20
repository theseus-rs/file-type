use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28770341: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_341,
        source_type: SourceType::Wikidata,
        name: "M2k",
        extensions: &["m2k"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
