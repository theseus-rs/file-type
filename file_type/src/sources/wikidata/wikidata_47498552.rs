use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47498552: FileType = FileType {
    file_format: &FileFormat {
        id: 47_498_552,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator file format, version 11",
        extensions: &["ai", "pdf"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
