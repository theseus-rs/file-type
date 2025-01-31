use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28049588: FileFormat = FileFormat {
    id: 28_049_588,
    puid: "wikidata/28049588",
    name: "Tiny Stuff, low resolution",
    extensions: &["tn1", "tny"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
