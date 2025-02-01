use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111284142: FileFormat = FileFormat {
    id: 111_284_142,
    puid: "wikidata/111284142",
    name: "G.726 5-bit (40 kbps) ADPCM format data",
    extensions: &["g726-5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
