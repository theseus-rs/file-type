use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_121840625: FileFormat = FileFormat {
    id: 121_840_625,
    puid: "wikidata/121840625",
    name: "Common Instrument File 1",
    extensions: &["ci1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
