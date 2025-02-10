use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129998017: FileType = FileType {
    file_format: &FileFormat {
        id: 129_998_017,
        source_type: SourceType::Wikidata,
        name: "JSON query and transformation language file format",
        extensions: &["jslt"],
        media_types: &["text/x-jslt"],
        signatures: &[],
        related_formats: &[],
    },
};
