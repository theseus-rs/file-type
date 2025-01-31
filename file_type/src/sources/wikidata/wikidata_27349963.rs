use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27349963: FileFormat = FileFormat {
    id: 27_349_963,
    puid: "wikidata/27349963",
    name: "TOPSAR C-Band VV Data",
    extensions: &["vvi2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
