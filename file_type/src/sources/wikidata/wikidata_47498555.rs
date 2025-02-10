use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47498555: FileType = FileType {
    file_format: &FileFormat {
        id: 47_498_555,
        source_type: SourceType::Wikidata,
        name: "Adobe Illustrator file format, version 12",
        extensions: &["ai", "pdf"],
        media_types: &["application/postscript"],
        signatures: &[],
        related_formats: &[],
    },
};
