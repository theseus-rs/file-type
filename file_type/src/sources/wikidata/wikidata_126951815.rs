use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_126951815: FileFormat = FileFormat {
    id: 126_951_815,
    source_type: SourceType::Wikidata,
    name: "Rust source code file",
    extensions: &["rs"],
    media_types: &["text/rust", "text/x-rust"],
    internal_signatures: &[],
    related_formats: &[],
};
