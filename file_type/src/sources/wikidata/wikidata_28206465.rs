use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206465: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_465,
        source_type: SourceType::Wikidata,
        name: "KoalaPainter uncompressed",
        extensions: &["koa"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
