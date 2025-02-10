use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206471: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_471,
        source_type: SourceType::Wikidata,
        name: "KoalaPainter compressed",
        extensions: &["gg"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
