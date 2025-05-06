use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134285328: FileType = FileType {
    file_format: &FileFormat {
        id: 134_285_328,
        source_type: SourceType::Wikidata,
        name: "Clipper preprocessor file",
        extensions: &["ppo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
