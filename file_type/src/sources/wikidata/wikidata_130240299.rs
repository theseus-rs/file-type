use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130240299: FileFormat = FileFormat {
    id: 130_240_299,
    puid: "wikidata/130240299",
    name: "Literate Agda source code file",
    extensions: &["lagda"],
    media_types: &["text/x-literate-agda"],
    internal_signatures: &[],
    related_formats: &[],
};
