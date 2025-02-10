use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47233167: FileType = FileType {
    file_format: &FileFormat {
        id: 47_233_167,
        source_type: SourceType::Wikidata,
        name: "LDR",
        extensions: &["dat", "ldr"],
        media_types: &["application/x-ldraw"],
        signatures: &[],
        related_formats: &[],
    },
};
