use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_131466271: FileType = FileType {
    file_format: &FileFormat {
        id: 131_466_271,
        source_type: SourceType::Wikidata,
        name: "Guy’s Image Processing Lab file format",
        extensions: &["gipl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
