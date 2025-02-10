use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27967085: FileType = FileType {
    file_format: &FileFormat {
        id: 27_967_085,
        source_type: SourceType::Wikidata,
        name: "Jason Page",
        extensions: &["jpn", "jpo", "smp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
