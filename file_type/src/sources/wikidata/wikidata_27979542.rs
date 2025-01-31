use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979542: FileFormat = FileFormat {
    id: 27_979_542,
    puid: "wikidata/27979542",
    name: "BookmarkData",
    extensions: &["sfl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
