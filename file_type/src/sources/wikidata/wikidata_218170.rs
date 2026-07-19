use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_218170: FileType = FileType {
    file_format: &FileFormat {
        id: 218_170,
        source_type: SourceType::Wikidata,
        name: "Q218170",
        extensions: &["ps"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
