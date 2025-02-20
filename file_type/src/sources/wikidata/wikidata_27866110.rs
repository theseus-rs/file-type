use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27866110: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_110,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator Artwork, version 8.0",
        extensions: &["ai"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
