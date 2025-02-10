use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125134559: FileType = FileType {
    file_format: &FileFormat {
        id: 125_134_559,
        source_type: SourceType::Wikidata,
        name: "YAM Dictionary",
        extensions: &["glossary"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
