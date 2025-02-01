use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_59390872: FileFormat = FileFormat {
    id: 59_390_872,
    puid: "wikidata/59390872",
    name: "GraphPad Prism file format",
    extensions: &["pzm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
