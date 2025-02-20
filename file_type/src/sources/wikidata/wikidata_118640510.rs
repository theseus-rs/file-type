use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
