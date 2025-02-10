use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27866120: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_120,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator Artwork, version 6.0",
        extensions: &["ai"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
