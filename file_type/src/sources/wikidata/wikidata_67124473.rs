use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_67124473: FileType = FileType {
    file_format: &FileFormat {
        id: 67_124_473,
        source_type: SourceType::Wikidata,
        name: "Print Artist letterhead file format",
        extensions: &["lth"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
