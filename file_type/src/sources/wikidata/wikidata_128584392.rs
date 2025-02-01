use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128584392: FileFormat = FileFormat {
    id: 128_584_392,
    puid: "wikidata/128584392",
    name: "ABNF file format",
    extensions: &["abnf"],
    media_types: &["text/x-abnf"],
    internal_signatures: &[],
    related_formats: &[],
};
