use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67126392: FileFormat = FileFormat {
    id: 67_126_392,
    puid: "wikidata/67126392",
    name: "Print Artist quote file format",
    extensions: &["qot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
