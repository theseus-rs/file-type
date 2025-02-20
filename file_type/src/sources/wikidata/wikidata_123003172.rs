use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_123003172: FileType = FileType {
    file_format: &FileFormat {
        id: 123_003_172,
        source_type: SourceType::Wikidata,
        name: "Truevision TGA 1.0",
        extensions: &["icb", "tga", "vda", "vst"],
        media_types: &["image/x-targa", "image/x-tga"],
        signatures: &[],
        related_formats: &[],
    },
};
