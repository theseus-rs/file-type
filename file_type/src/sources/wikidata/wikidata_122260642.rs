use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122260642: FileType = FileType {
    file_format: &FileFormat {
        id: 122_260_642,
        source_type: SourceType::Wikidata,
        name: "JACOsub",
        extensions: &["jss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
