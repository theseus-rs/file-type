use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26207815: FileType = FileType {
    file_format: &FileFormat {
        id: 26_207_815,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Presentation Document, Transitional, ISO/IEC 29500:2012",
        extensions: &["pptx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
