use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167888: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_888,
        source_type: SourceType::Wikidata,
        name: "Personal Ancestral File, version 3",
        extensions: &["paf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
