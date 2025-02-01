use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967422: FileFormat = FileFormat {
    id: 27_967_422,
    puid: "wikidata/27967422",
    name: "ChordML",
    extensions: &["cml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
