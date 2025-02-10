use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117324669: FileType = FileType {
    file_format: &FileFormat {
        id: 117_324_669,
        source_type: SourceType::Wikidata,
        name: "LabWindows/CVI Project file",
        extensions: &["prj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
