use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_1566078: FileType = FileType {
    file_format: &FileFormat {
        id: 1_566_078,
        source_type: SourceType::Wikidata,
        name: "HTML Application",
        extensions: &["hta"],
        media_types: &["application/hta"],
        signatures: &[],
        related_formats: &[],
    },
};
