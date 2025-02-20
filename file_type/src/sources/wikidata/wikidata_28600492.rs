use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600492: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_492,
        source_type: SourceType::Wikidata,
        name: "Data Resource File",
        extensions: &["drs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
