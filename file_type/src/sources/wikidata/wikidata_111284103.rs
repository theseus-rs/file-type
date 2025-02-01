use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111284103: FileFormat = FileFormat {
    id: 111_284_103,
    puid: "wikidata/111284103",
    name: "G.726 3-bit (24 kbps) ADPCM format data",
    extensions: &["g726-3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
