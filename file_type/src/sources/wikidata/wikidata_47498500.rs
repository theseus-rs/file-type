use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47498500: FileType = FileType {
    file_format: &FileFormat {
        id: 47_498_500,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator file format, version 8",
        extensions: &["ai"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
