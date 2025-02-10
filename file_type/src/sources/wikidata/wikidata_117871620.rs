use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117871620: FileType = FileType {
    file_format: &FileFormat {
        id: 117_871_620,
        source_type: SourceType::Wikidata,
        name: "U.S. Robotics WorldPort 2496 file",
        extensions: &["wpf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
