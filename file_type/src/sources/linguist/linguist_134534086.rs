use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_134534086: FileFormat = FileFormat {
    id: 134_534_086,
    source_type: SourceType::Linguist,
    name: "WebAssembly Interface Type",
    extensions: &["wit"],
    media_types: &["text/x-webidl"],
    internal_signatures: &[],
    related_formats: &[],
};
