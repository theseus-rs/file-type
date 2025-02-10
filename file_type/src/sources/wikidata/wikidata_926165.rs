use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_926165: FileType = FileType {
    file_format: &FileFormat {
        id: 926_165,
        source_type: SourceType::Wikidata,
        name: "Geography Markup Language",
        extensions: &["gml"],
        media_types: &["application/gml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
