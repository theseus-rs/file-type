use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130293828: FileFormat = FileFormat {
    id: 130_293_828,
    source_type: SourceType::Wikidata,
    name: "MiniScript source code file",
    extensions: &["ms"],
    media_types: &["application/x-miniscript", "text/x-miniscript"],
    signatures: &[],
    related_formats: &[],
};
