use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28770290: FileType = FileType {
    file_format: &FileFormat {
        id: 28_770_290,
        source_type: SourceType::Wikidata,
        name: "LSB",
        extensions: &["lsb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
