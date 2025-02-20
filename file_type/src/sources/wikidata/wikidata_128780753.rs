use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_128780753: FileType = FileType {
    file_format: &FileFormat {
        id: 128_780_753,
        source_type: SourceType::Wikidata,
        name: "crmsh configuration file",
        extensions: &["crmsh"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
