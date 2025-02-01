use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_64763203: FileFormat = FileFormat {
    id: 64_763_203,
    puid: "wikidata/64763203",
    name: "MapPoint template file format",
    extensions: &["ptt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
