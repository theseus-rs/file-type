use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116958386: FileType = FileType {
    file_format: &FileFormat {
        id: 116_958_386,
        source_type: SourceType::Wikidata,
        name: "Pegasus PIC",
        extensions: &["pic"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
