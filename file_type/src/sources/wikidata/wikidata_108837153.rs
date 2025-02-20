use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_108837153: FileType = FileType {
    file_format: &FileFormat {
        id: 108_837_153,
        source_type: SourceType::Wikidata,
        name: "Quicken v4 Data File",
        extensions: &["qdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
