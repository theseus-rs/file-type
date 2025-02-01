use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118139110: FileFormat = FileFormat {
    id: 118_139_110,
    puid: "wikidata/118139110",
    name: "Calendar Creator 2.x Event File",
    extensions: &["cee"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
