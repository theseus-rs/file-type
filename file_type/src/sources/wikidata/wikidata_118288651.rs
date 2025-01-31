use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118288651: FileFormat = FileFormat {
    id: 118_288_651,
    puid: "wikidata/118288651",
    name: "OnMark 2000 Project File",
    extensions: &["era"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
