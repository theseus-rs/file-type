use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111284090: FileFormat = FileFormat {
    id: 111_284_090,
    puid: "wikidata/111284090",
    name: "G.723 3-bit (24 kbps) ADPCM format data",
    extensions: &["g723-3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
