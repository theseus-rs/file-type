use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47165998: FileType = FileType {
    file_format: &FileFormat {
        id: 47_165_998,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks file format version 1",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
