use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27866113: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_113,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator Artwork, version 4.0",
        extensions: &["ai"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
