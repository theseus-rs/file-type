use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1921702: FileType = FileType {
    file_format: &FileFormat {
        id: 1_921_702,
        source_type: SourceType::Wikidata,
        name: "Meridian Lossless Packing",
        extensions: &[],
        media_types: &["audio/vnd.dolby.mlp"],
        signatures: &[],
        related_formats: &[],
    },
};
