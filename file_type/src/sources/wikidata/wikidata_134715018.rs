use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134715018: FileType = FileType {
    file_format: &FileFormat {
        id: 134_715_018,
        source_type: SourceType::Wikidata,
        name: "Seed7 program file",
        extensions: &["sd7"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
