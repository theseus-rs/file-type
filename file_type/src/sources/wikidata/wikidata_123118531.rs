use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_123118531: FileType = FileType {
    file_format: &FileFormat {
        id: 123_118_531,
        source_type: SourceType::Wikidata,
        name: "PostScript 3.1",
        extensions: &["ps"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
