use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27349804: FileFormat = FileFormat {
    id: 27_349_804,
    puid: "wikidata/27349804",
    name: "ESRI Arc/Info Binary Grid",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
