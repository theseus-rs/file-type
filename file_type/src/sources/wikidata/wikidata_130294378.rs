use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130294378: FileFormat = FileFormat {
    id: 130_294_378,
    puid: "wikidata/130294378",
    name: "MIPS file format",
    extensions: &["mips"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
