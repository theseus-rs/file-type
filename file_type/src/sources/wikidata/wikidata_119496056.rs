use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119496056: FileType = FileType {
    file_format: &FileFormat {
        id: 119_496_056,
        source_type: SourceType::Wikidata,
        name: "IBM IOCA Raw",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
