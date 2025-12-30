use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136678671: FileType = FileType {
    file_format: &FileFormat {
        id: 136_678_671,
        source_type: SourceType::Wikidata,
        name: "Comic Life file",
        extensions: &["comicdoc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
