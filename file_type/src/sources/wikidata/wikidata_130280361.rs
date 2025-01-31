use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130280361: FileFormat = FileFormat {
    id: 130_280_361,
    puid: "wikidata/130280361",
    name: "Mason file format",
    extensions: &["m"],
    media_types: &["application/x-mason"],
    internal_signatures: &[],
    related_formats: &[],
};
