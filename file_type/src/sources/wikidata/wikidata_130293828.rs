use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130293828: FileFormat = FileFormat {
    id: 130_293_828,
    puid: "wikidata/130293828",
    name: "MiniScript source code file",
    extensions: &["ms", "ms"],
    media_types: &["application/x-miniscript", "text/x-miniscript"],
    internal_signatures: &[],
    related_formats: &[],
};
