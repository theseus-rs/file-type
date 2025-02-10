use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206749: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_749,
        source_type: SourceType::Wikidata,
        name: "Native Image File Format",
        extensions: &["niff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
