use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111284095: FileFormat = FileFormat {
    id: 111_284_095,
    puid: "wikidata/111284095",
    name: "G.723 5-bit (40 kbps) ADPCM format data",
    extensions: &["g723-5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
