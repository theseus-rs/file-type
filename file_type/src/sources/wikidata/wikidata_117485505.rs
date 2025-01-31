use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117485505: FileFormat = FileFormat {
    id: 117_485_505,
    puid: "wikidata/117485505",
    name: "MacCaption Project",
    extensions: &["cca"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
