use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_66689214: FileType = FileType {
    file_format: &FileFormat {
        id: 66_689_214,
        source_type: SourceType::Wikidata,
        name: "Access Blank Database Template",
        extensions: &["mdn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
