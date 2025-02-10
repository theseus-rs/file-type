use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_125134354: FileType = FileType {
    file_format: &FileFormat {
        id: 125_134_354,
        source_type: SourceType::Wikidata,
        name: "YAM Folder Configuration",
        extensions: &["fconfig"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
