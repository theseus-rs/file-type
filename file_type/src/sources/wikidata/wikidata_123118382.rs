use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123118382: FileType = FileType {
    file_format: &FileFormat {
        id: 123_118_382,
        source_type: SourceType::Wikidata,
        name: "PostScript 1.0",
        extensions: &["ps"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
