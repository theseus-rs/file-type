use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26207802: FileType = FileType {
    file_format: &FileFormat {
        id: 26_207_802,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Wordprocessing Document, Transitional, ISO/IEC 29500:2012",
        extensions: &["docx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
        signatures: &[],
        related_formats: &[],
    },
};
