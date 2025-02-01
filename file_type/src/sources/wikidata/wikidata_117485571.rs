use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117485571: FileFormat = FileFormat {
    id: 117_485_571,
    puid: "wikidata/117485571",
    name: "Audacity Audio Block File",
    extensions: &["au"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
