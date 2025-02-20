use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_13039854: FileType = FileType {
    file_format: &FileFormat {
        id: 13_039_854,
        source_type: SourceType::Wikidata,
        name: "C++ header",
        extensions: &["h", "hh", "hpp", "hxx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
