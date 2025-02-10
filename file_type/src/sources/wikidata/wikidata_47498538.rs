use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47498538: FileType = FileType {
    file_format: &FileFormat {
        id: 47_498_538,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator file format, version 9.0",
        extensions: &["ai", "pdf"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
