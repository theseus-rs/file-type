use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47896997: FileFormat = FileFormat {
    id: 47_896_997,
    puid: "wikidata/47896997",
    name: "Drawing Interchange Format Style Extract",
    extensions: &["dxx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
