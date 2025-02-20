use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26207824: FileType = FileType {
    file_format: &FileFormat {
        id: 26_207_824,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Presentation Document, Strict, ISO/IEC 29500:2012",
        extensions: &["pptx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
