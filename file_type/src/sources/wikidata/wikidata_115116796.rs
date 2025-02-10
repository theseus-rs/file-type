use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_115116796: FileType = FileType {
    file_format: &FileFormat {
        id: 115_116_796,
        source_type: SourceType::Wikidata,
        name: "Gunpaint Image File",
        extensions: &["gun"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
