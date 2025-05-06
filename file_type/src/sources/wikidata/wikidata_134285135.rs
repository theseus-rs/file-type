use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134285135: FileType = FileType {
    file_format: &FileFormat {
        id: 134_285_135,
        source_type: SourceType::Wikidata,
        name: "Clipper temporary file",
        extensions: &["tmp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
