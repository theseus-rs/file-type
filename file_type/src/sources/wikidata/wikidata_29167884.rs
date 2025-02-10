use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167884: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_884,
        source_type: SourceType::Wikidata,
        name: "Personal Ancestral File",
        extensions: &["paf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
