use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27824060: FileFormat = FileFormat {
    id: 27_824_060,
    puid: "wikidata/27824060",
    name: "Internet Archive ARC, version 1.0",
    extensions: &["arc"],
    media_types: &["application/x-internet-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
