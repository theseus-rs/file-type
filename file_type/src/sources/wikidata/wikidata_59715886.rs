use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_59715886: FileType = FileType {
    file_format: &FileFormat {
        id: 59_715_886,
        source_type: SourceType::Wikidata,
        name: "CALS Compressed Bitmap",
        extensions: &["cal"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
