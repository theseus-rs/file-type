use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_184: FileFormat = FileFormat {
    id: 184,
    source_type: SourceType::Linguist,
    name: "Julia",
    extensions: &["jl"],
    media_types: &["text/x-julia"],
    internal_signatures: &[],
    related_formats: &[],
};
