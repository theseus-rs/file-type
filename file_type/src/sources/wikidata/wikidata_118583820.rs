use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118583820: FileType = FileType {
    file_format: &FileFormat {
        id: 118_583_820,
        source_type: SourceType::Wikidata,
        name: "Cakewalk Project",
        extensions: &["cwp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
