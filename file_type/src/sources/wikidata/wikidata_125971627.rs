use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125971627: FileType = FileType {
    file_format: &FileFormat {
        id: 125_971_627,
        source_type: SourceType::Wikidata,
        name: "gemtext",
        extensions: &["gmi"],
        media_types: &["text/gemini"],
        signatures: &[],
        related_formats: &[],
    },
};
