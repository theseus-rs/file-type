use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_112653369: FileType = FileType {
    file_format: &FileFormat {
        id: 112_653_369,
        source_type: SourceType::Wikidata,
        name: "Astound Draw backup file",
        extensions: &["ad~"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
