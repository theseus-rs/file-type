use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47520788: FileType = FileType {
    file_format: &FileFormat {
        id: 47_520_788,
        source_type: SourceType::Wikidata,
        name: "Serif PagePlus Publication file format, version 10",
        extensions: &["ppp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
