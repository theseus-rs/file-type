use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110039945: FileFormat = FileFormat {
    id: 110_039_945,
    puid: "wikidata/110039945",
    name: "Phantom CINE Video File",
    extensions: &["cin", "cine"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
