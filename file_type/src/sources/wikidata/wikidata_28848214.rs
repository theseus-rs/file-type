use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28848214: FileType = FileType {
    file_format: &FileFormat {
        id: 28_848_214,
        source_type: SourceType::Wikidata,
        name: "Statistical Package for the Social Sciences data file",
        extensions: &["sav"],
        media_types: &["application/x-spss-sav"],
        signatures: &[],
        related_formats: &[],
    },
};
