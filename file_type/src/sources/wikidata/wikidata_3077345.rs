use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_3077345: FileType = FileType {
    file_format: &FileFormat {
        id: 3_077_345,
        source_type: SourceType::Wikidata,
        name: "Polygon File Format",
        extensions: &["ply"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
