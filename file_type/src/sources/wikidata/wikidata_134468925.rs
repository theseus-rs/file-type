use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134468925: FileType = FileType {
    file_format: &FileFormat {
        id: 134_468_925,
        source_type: SourceType::Wikidata,
        name: "Flat Assembler file",
        extensions: &["fasm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
