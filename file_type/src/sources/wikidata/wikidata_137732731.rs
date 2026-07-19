use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_137732731: FileType = FileType {
    file_format: &FileFormat {
        id: 137_732_731,
        source_type: SourceType::Wikidata,
        name: "Adobe Captivate template file",
        extensions: &["cptl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
