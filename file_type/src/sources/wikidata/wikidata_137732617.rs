use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137732617: FileType = FileType {
    file_format: &FileFormat {
        id: 137_732_617,
        source_type: SourceType::Wikidata,
        name: "Adobe Captivate 5 project file",
        extensions: &["cptx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
