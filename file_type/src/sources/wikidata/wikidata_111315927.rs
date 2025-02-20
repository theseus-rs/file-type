use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
