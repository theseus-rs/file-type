use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130618874: FileFormat = FileFormat {
    id: 130_618_874,
    puid: "wikidata/130618874",
    name: "Redcode file format",
    extensions: &["cw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
