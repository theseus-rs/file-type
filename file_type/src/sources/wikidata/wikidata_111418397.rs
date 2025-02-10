use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111418397: FileType = FileType {
    file_format: &FileFormat {
        id: 111_418_397,
        source_type: SourceType::Wikidata,
        name: "Adobe Bridge Cache Export File",
        extensions: &["bridgecache"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
