use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50288247: FileType = FileType {
    file_format: &FileFormat {
        id: 50_288_247,
        source_type: SourceType::Wikidata,
        name: "Adobe Air, v2",
        extensions: &["air"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
