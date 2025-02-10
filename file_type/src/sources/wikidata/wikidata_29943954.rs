use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_29943954: FileType = FileType {
    file_format: &FileFormat {
        id: 29_943_954,
        source_type: SourceType::Wikidata,
        name: "Statistical Package for the Social Sciences output file",
        extensions: &["spv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
