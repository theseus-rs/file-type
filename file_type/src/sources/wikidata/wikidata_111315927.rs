use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111315927: FileType = FileType {
    file_format: &FileFormat {
        id: 111_315_927,
        source_type: SourceType::Wikidata,
        name: "Ensoniq EPS family instrument",
        extensions: &["ins"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
