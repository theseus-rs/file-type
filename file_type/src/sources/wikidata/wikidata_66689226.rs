use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_66689226: FileType = FileType {
    file_format: &FileFormat {
        id: 66_689_226,
        source_type: SourceType::Wikidata,
        name: "Access Add-in Data",
        extensions: &["mdt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
