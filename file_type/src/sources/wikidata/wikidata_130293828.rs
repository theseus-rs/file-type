use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130293828: FileFormat = FileFormat {
    id: 130_293_828,
    source_type: SourceType::Wikidata,
    name: "MiniScript source code file",
    extensions: &["ms"],
    media_types: &["application/x-miniscript", "text/x-miniscript"],
    internal_signatures: &[],
    related_formats: &[],
};
