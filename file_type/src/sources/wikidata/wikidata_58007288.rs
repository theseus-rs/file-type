use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_58007288: FileType = FileType {
    file_format: &FileFormat {
        id: 58_007_288,
        source_type: SourceType::Wikidata,
        name: "VBScript file",
        extensions: &["vbs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
