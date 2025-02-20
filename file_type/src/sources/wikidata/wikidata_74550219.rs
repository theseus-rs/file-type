use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_74550219: FileType = FileType {
    file_format: &FileFormat {
        id: 74_550_219,
        source_type: SourceType::Wikidata,
        name: "Micrografx clipart index",
        extensions: &["sbj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
