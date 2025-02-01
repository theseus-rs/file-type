use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47202816: FileFormat = FileFormat {
    id: 47_202_816,
    puid: "wikidata/47202816",
    name: "AppleWorks Painting file format version 6",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
