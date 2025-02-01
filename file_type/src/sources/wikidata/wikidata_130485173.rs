use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130485173: FileFormat = FileFormat {
    id: 130_485_173,
    puid: "wikidata/130485173",
    name: "Portugol file format",
    extensions: &["alg", "portugol"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
