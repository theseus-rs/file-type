use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110238835: FileType = FileType {
    file_format: &FileFormat {
        id: 110_238_835,
        source_type: SourceType::Wikidata,
        name: "Gorilla Scheduling",
        extensions: &["sex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
