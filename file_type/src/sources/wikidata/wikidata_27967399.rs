use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967399: FileFormat = FileFormat {
    id: 27_967_399,
    puid: "wikidata/27967399",
    name: "AMusic module",
    extensions: &["amd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
