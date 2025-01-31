use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110502531: FileFormat = FileFormat {
    id: 110_502_531,
    puid: "wikidata/110502531",
    name: "ISDOCX Information System Document (Generic)",
    extensions: &["isdocx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
