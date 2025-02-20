use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27866112: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_112,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator Artwork, version 3.0/3.2",
        extensions: &["ai"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
