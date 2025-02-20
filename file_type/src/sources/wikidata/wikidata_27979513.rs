use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979513: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_513,
        source_type: SourceType::Wikidata,
        name: "Manga Studio Story",
        extensions: &["cst"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
