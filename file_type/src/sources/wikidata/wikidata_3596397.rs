use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_3596397: FileType = FileType {
    file_format: &FileFormat {
        id: 3_596_397,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Presentation Document, ECMA-376 1st Edition",
        extensions: &["pptx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
