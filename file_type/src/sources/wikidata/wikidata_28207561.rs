use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207561: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_561,
        source_type: SourceType::Wikidata,
        name: "Xim",
        extensions: &["xim"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
