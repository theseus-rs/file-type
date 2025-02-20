use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131278668: FileType = FileType {
    file_format: &FileFormat {
        id: 131_278_668,
        source_type: SourceType::Wikidata,
        name: "Turbo Assembler assembly code file",
        extensions: &["asm", "tasm"],
        media_types: &["text/x-tasm"],
        signatures: &[],
        related_formats: &[],
    },
};
