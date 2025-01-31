use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_113578334: FileFormat = FileFormat {
    id: 113_578_334,
    puid: "wikidata/113578334",
    name: "Music Maker Arrangement File",
    extensions: &["mmm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
