use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131621225: FileFormat = FileFormat {
    id: 131_621_225,
    puid: "wikidata/131621225",
    name: "Dyna database file format",
    extensions: &["d3plot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
