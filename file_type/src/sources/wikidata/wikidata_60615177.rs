use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_60615177: FileType = FileType {
    file_format: &FileFormat {
        id: 60_615_177,
        source_type: SourceType::Wikidata,
        name: "Serif DrawPlus Drawing, version 5",
        extensions: &["dpp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
