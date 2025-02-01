use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111284134: FileFormat = FileFormat {
    id: 111_284_134,
    puid: "wikidata/111284134",
    name: "G.726 4-bit (32 kbps) ADPCM format data",
    extensions: &["g726-4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
