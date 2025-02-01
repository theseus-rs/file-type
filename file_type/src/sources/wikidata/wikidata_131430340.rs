use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131430340: FileFormat = FileFormat {
    id: 131_430_340,
    puid: "wikidata/131430340",
    name: "X10 file format",
    extensions: &["x10"],
    media_types: &["text/x-x10"],
    internal_signatures: &[],
    related_formats: &[],
};
