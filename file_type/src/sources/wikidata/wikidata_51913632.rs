use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51913632: FileType = FileType {
    file_format: &FileFormat {
        id: 51_913_632,
        source_type: SourceType::Wikidata,
        name: "SDSC Image Tool Run-Length Encoded Bitmap",
        extensions: &["rle"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
