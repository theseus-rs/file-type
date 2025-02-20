use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_184: FileType = FileType {
    file_format: &FileFormat {
        id: 184,
        source_type: SourceType::Linguist,
        name: "Julia",
        extensions: &["jl"],
        media_types: &["text/x-julia"],
        signatures: &[],
        related_formats: &[],
    },
};
