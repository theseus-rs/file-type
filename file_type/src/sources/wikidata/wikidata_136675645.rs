use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136675645: FileType = FileType {
    file_format: &FileFormat {
        id: 136_675_645,
        source_type: SourceType::Wikidata,
        name: "ConceptDraw Library",
        extensions: &["cdl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
