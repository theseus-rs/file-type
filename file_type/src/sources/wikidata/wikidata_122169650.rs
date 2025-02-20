use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_122169650: FileType = FileType {
    file_format: &FileFormat {
        id: 122_169_650,
        source_type: SourceType::Wikidata,
        name: "Password Cache File",
        extensions: &["epc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
