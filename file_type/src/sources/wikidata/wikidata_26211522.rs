use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26211522: FileType = FileType {
    file_format: &FileFormat {
        id: 26_211_522,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Wordprocessing Document, Transitional, ISO/IEC 29500:2012, with Microsoft extensions",
        extensions: &["docx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
        signatures: &[],
        related_formats: &[],
    },
};
