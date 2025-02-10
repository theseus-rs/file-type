use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_109596500: FileType = FileType {
    file_format: &FileFormat {
        id: 109_596_500,
        source_type: SourceType::Wikidata,
        name: "DrawPlus Animation",
        extensions: &["dpa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
