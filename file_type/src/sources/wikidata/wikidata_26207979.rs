use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26207979: FileType = FileType {
    file_format: &FileFormat {
        id: 26_207_979,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Wordprocessing Document, Transitional, ISO/IEC 29500:2008, with Microsoft extensions",
        extensions: &["docx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
        signatures: &[],
        related_formats: &[],
    },
};
