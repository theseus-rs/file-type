use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_127380453: FileType = FileType {
    file_format: &FileFormat {
        id: 127_380_453,
        source_type: SourceType::Wikidata,
        name: "Vulnerability Exploitability eXchange file",
        extensions: &["vex"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
