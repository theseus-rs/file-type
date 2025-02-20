use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118721205: FileType = FileType {
    file_format: &FileFormat {
        id: 118_721_205,
        source_type: SourceType::Wikidata,
        name: "PZZ File",
        extensions: &["pzz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
