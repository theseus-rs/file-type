use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123003201: FileType = FileType {
    file_format: &FileFormat {
        id: 123_003_201,
        source_type: SourceType::Wikidata,
        name: "Truevision TGA 2.0",
        extensions: &["icb", "tga", "vda", "vst"],
        media_types: &["image/x-targa", "image/x-tga"],
        signatures: &[],
        related_formats: &[],
    },
};
