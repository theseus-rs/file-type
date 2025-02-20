use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_51370612: FileType = FileType {
    file_format: &FileFormat {
        id: 51_370_612,
        source_type: SourceType::Wikidata,
        name: "Inkwriter/Notetaker Document",
        extensions: &["pwi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
