use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_26208271: FileType = FileType {
    file_format: &FileFormat {
        id: 26_208_271,
        source_type: SourceType::Wikidata,
        name: "Office Open XML Presentation Document, Strict, ISO/IEC 29500:2008, with Microsoft extensions",
        extensions: &["pptx"],
        media_types: &["application/vnd.openxmlformats-officedocument.presentationml.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
