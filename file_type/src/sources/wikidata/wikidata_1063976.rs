use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1063976: FileType = FileType {
    file_format: &FileFormat {
        id: 1_063_976,
        source_type: SourceType::Wikidata,
        name: "Truevision TGA",
        extensions: &["icb", "tga", "tpic", "vda", "vst"],
        media_types: &["image/x-targa", "image/x-tga"],
        signatures: &[],
        related_formats: &[],
    },
};
