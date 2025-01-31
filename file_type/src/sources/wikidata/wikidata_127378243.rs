use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127378243: FileFormat = FileFormat {
    id: 127_378_243,
    puid: "wikidata/127378243",
    name: "FreeBASIC Header File",
    extensions: &["bi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
