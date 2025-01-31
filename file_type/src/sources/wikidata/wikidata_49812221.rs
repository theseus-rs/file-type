use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49812221: FileFormat = FileFormat {
    id: 49_812_221,
    puid: "wikidata/49812221",
    name: "Vectorworks file format, version 2009",
    extensions: &["vwx"],
    media_types: &["application/vnd.vectorworks"],
    internal_signatures: &[],
    related_formats: &[],
};
