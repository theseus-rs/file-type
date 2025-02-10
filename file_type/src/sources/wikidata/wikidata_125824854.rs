use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125824854: FileType = FileType {
    file_format: &FileFormat {
        id: 125_824_854,
        source_type: SourceType::Wikidata,
        name: "Tar Zipped File",
        extensions: &["taz"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
