use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27960000: FileType = FileType {
    file_format: &FileFormat {
        id: 27_960_000,
        source_type: SourceType::Wikidata,
        name: "Perfect Clarity Audio",
        extensions: &["pca"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
