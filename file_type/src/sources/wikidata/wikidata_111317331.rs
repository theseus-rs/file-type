use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111317331: FileFormat = FileFormat {
    id: 111_317_331,
    puid: "wikidata/111317331",
    name: "Native Instruments Reaktor format",
    extensions: &["map"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
