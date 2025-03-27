use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26207786: FileType = FileType {
    file_format: &FileFormat {
        id: 26_207_786,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Wordprocessing Document, Strict, ISO/IEC 29500:2011",
        extensions: &["docx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
        signatures: &[],
        related_formats: &[],
    },
};
