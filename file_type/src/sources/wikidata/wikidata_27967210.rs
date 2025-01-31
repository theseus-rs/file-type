use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967210: FileFormat = FileFormat {
    id: 27_967_210,
    puid: "wikidata/27967210",
    name: "Poly Tracker module",
    extensions: &["ptm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
