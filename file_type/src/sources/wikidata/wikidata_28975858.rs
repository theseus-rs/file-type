use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28975858: FileType = FileType {
    file_format: &FileFormat {
        id: 28_975_858,
        source_type: SourceType::Wikidata,
        name: "OOGL QUAD file",
        extensions: &["quad"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
