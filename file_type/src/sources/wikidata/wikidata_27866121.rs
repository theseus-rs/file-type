use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27866121: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_121,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator Artwork file format, version 7.0",
        extensions: &["ai"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
