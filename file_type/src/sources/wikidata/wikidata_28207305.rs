use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207305: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_305,
        source_type: SourceType::Wikidata,
        name: "True Colour Picture",
        extensions: &["trp", "tru"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
