use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206465: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_465,
        source_type: SourceType::Wikidata,
        name: "KoalaPainter uncompressed",
        extensions: &["koa"],
        media_types: &["image/x-koa", "image/x-koala"],
        signatures: &[],
        related_formats: &[],
    },
};
