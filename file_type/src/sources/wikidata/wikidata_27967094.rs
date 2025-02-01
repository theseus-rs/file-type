use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967094: FileFormat = FileFormat {
    id: 27_967_094,
    puid: "wikidata/27967094",
    name: "Interplay ACM",
    extensions: &["acm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
