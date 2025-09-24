use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136165258: FileType = FileType {
    file_format: &FileFormat {
        id: 136_165_258,
        source_type: SourceType::Wikidata,
        name: "Serif Transparent Video file",
        extensions: &["stv"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
