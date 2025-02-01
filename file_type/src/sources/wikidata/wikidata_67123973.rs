use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67123973: FileFormat = FileFormat {
    id: 67_123_973,
    puid: "wikidata/67123973",
    name: "Print Artist certificate file format",
    extensions: &["cer"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
