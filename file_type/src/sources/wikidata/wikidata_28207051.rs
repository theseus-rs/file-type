use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207051: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_051,
        source_type: SourceType::Wikidata,
        name: "Pocket PC Bitmap",
        extensions: &["2bp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
