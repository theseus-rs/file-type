use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_126951550: FileFormat = FileFormat {
    id: 126_951_550,
    puid: "wikidata/126951550",
    name: "J script file",
    extensions: &["ijs"],
    media_types: &["text/x-j"],
    internal_signatures: &[],
    related_formats: &[],
};
