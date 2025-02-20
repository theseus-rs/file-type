use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27866092: FileType = FileType {
    file_format: &FileFormat {
        id: 27_866_092,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator Artwork, version 1.0/1.1",
        extensions: &["ai"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
