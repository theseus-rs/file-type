use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206198: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_198,
        source_type: SourceType::Wikidata,
        name: "GodPaint",
        extensions: &["god"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
