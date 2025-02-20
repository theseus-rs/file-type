use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979516: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_516,
        source_type: SourceType::Wikidata,
        name: "Manga Studio Page",
        extensions: &["cpg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
