use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27824065: FileFormat = FileFormat {
    id: 27_824_065,
    puid: "wikidata/27824065",
    name: "Internet Archive ARC, version 1.1",
    extensions: &["arc"],
    media_types: &["application/x-internet-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
