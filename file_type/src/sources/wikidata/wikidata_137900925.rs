use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137900925: FileType = FileType {
    file_format: &FileFormat {
        id: 137_900_925,
        source_type: SourceType::Wikidata,
        name: "Ngspice data file",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
