use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111512376: FileType = FileType {
    file_format: &FileFormat {
        id: 111_512_376,
        source_type: SourceType::Wikidata,
        name: "ASEG-GDF2- Data definition file",
        extensions: &["dfn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
