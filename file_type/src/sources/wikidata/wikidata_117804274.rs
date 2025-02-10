use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117804274: FileType = FileType {
    file_format: &FileFormat {
        id: 117_804_274,
        source_type: SourceType::Wikidata,
        name: "VideoImpression mini-player",
        extensions: &["exe"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
