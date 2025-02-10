use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118640510: FileType = FileType {
    file_format: &FileFormat {
        id: 118_640_510,
        source_type: SourceType::Wikidata,
        name: "Anime Studio File",
        extensions: &["anme"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
