use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117324987: FileType = FileType {
    file_format: &FileFormat {
        id: 117_324_987,
        source_type: SourceType::Wikidata,
        name: "LabVIEW control",
        extensions: &["ctl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
