use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3477565: FileFormat = FileFormat {
    id: 3_477_565,
    puid: "wikidata/3477565",
    name: "Secure Digital Container",
    extensions: &["sdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
