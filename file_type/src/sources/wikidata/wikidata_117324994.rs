use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117324994: FileType = FileType {
    file_format: &FileFormat {
        id: 117_324_994,
        source_type: SourceType::Wikidata,
        name: "LabVIEW control template",
        extensions: &["ctt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
