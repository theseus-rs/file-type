use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29167891: FileType = FileType {
    file_format: &FileFormat {
        id: 29_167_891,
        source_type: SourceType::Wikidata,
        name: "Personal Ancestral File, version 4",
        extensions: &["paf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
