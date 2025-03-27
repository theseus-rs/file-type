use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_29024883: FileType = FileType {
    file_format: &FileFormat {
        id: 29_024_883,
        source_type: SourceType::Wikidata,
        name: ".spx",
        extensions: &[],
        media_types: &["audio/speex", "audio/x-speex"],
        signatures: &[],
        related_formats: &[],
    },
};
