use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111418328: FileType = FileType {
    file_format: &FileFormat {
        id: 111_418_328,
        source_type: SourceType::Wikidata,
        name: "Adobe Bridge Data File",
        extensions: &["abdata"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
