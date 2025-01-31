use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111284101: FileFormat = FileFormat {
    id: 111_284_101,
    puid: "wikidata/111284101",
    name: "G.726 2-bit (16 kbps) ADPCM format data",
    extensions: &["g726-2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
