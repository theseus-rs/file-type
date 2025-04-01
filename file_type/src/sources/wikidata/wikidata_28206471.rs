use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206471: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_471,
        source_type: SourceType::Wikidata,
        name: "KoalaPainter compressed",
        extensions: &["gg"],
        media_types: &["image/x-koa", "image/x-koala"],
        signatures: &[],
        related_formats: &[],
    },
};
