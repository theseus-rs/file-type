use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52063298: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_298,
        source_type: SourceType::Wikidata,
        name: "Scanstudio 16-Colour Bitmap",
        extensions: &["adc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
