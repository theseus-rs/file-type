use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_113382492: FileType = FileType {
    file_format: &FileFormat {
        id: 113_382_492,
        source_type: SourceType::Wikidata,
        name: "Roxio Easy Media Creator Classic Creator File 8-10",
        extensions: &["rcl"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
