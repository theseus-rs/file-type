use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111284097: FileFormat = FileFormat {
    id: 111_284_097,
    puid: "wikidata/111284097",
    name: "G.726 2/3/4/5-bit ADPCM format data",
    extensions: &["g726"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
