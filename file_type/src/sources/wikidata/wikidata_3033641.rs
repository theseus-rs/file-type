use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3033641: FileType = FileType {
    file_format: &FileFormat {
        id: 3_033_641,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Wordprocessing Document, ECMA-376 1st Edition",
        extensions: &["docx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
        signatures: &[],
        related_formats: &[],
    },
};
