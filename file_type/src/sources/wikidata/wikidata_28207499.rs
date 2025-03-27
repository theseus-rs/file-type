use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207499: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_499,
        source_type: SourceType::Wikidata,
        name: "Windows Device-Dependent Bitmap",
        extensions: &["bmp", "ddb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
