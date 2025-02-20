use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_10846524: FileType = FileType {
    file_format: &FileFormat {
        id: 10_846_524,
        source_type: SourceType::Wikidata,
        name: "Blu-ray Disk Movie",
        extensions: &["bdm", "bdmv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
