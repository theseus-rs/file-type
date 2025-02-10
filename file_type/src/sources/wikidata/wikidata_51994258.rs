use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51994258: FileType = FileType {
    file_format: &FileFormat {
        id: 51_994_258,
        source_type: SourceType::Wikidata,
        name: "DEC WPS Plus Document",
        extensions: &["wpl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
