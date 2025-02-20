use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130245180: FileType = FileType {
    file_format: &FileFormat {
        id: 130_245_180,
        source_type: SourceType::Wikidata,
        name: "LLVM assembly code file",
        extensions: &["ll"],
        media_types: &["text/x-llvm"],
        signatures: &[],
        related_formats: &[],
    },
};
