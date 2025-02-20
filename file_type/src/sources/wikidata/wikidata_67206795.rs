use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_67206795: FileType = FileType {
    file_format: &FileFormat {
        id: 67_206_795,
        source_type: SourceType::Wikidata,
        name: "SmartSketch Drawing",
        extensions: &["ssk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
