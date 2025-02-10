use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47167584: FileType = FileType {
    file_format: &FileFormat {
        id: 47_167_584,
        source_type: SourceType::Wikidata,
        name: "ClarisWorks Painting file format",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
