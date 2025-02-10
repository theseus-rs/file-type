use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117324972: FileType = FileType {
    file_format: &FileFormat {
        id: 117_324_972,
        source_type: SourceType::Wikidata,
        name: "LabVIEW virtual instrument template",
        extensions: &["vit"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
