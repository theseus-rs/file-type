use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59653819: FileType = FileType {
    file_format: &FileFormat {
        id: 59_653_819,
        source_type: SourceType::Wikidata,
        name: "Maya Binary File Format, 64 bit",
        extensions: &["mb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
