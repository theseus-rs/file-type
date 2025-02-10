use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_26207727: FileType = FileType {
    file_format: &FileFormat {
        id: 26_207_727,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Presentation Document, Strict, ISO/IEC 29500:2008",
        extensions: &["pptx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
