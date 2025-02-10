use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117352081: FileType = FileType {
    file_format: &FileFormat {
        id: 117_352_081,
        source_type: SourceType::Wikidata,
        name: "Capture Library",
        extensions: &["olb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
