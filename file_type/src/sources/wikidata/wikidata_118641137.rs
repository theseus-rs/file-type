use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118641137: FileType = FileType {
    file_format: &FileFormat {
        id: 118_641_137,
        source_type: SourceType::Wikidata,
        name: "WebM Audio",
        extensions: &["weba"],
        media_types: &["audio/webm"],
        signatures: &[],
        related_formats: &[],
    },
};
