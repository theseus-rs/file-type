use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_116861020: FileType = FileType {
    file_format: &FileFormat {
        id: 116_861_020,
        source_type: SourceType::Wikidata,
        name: "Disney Print Creations Project",
        extensions: &["dpc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
