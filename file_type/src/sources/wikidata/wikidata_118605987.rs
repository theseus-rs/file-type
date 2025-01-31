use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_118605987: FileFormat = FileFormat {
    id: 118_605_987,
    puid: "wikidata/118605987",
    name: "Visual J# File",
    extensions: &["jsl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
