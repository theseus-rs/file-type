use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130271417: FileFormat = FileFormat {
    id: 130_271_417,
    puid: "wikidata/130271417",
    name: "Mako file format",
    extensions: &["mao"],
    media_types: &["application/x-mako"],
    internal_signatures: &[],
    related_formats: &[],
};
