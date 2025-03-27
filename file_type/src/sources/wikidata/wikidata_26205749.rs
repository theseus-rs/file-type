use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26205749: FileType = FileType {
    file_format: &FileFormat {
        id: 26_205_749,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Wordprocessing Document, Transitional, ISO/IEC 29500:2008",
        extensions: &["docx"],
        media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
        signatures: &[],
        related_formats: &[],
    },
};
