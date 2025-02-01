use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205601: FileFormat = FileFormat {
    id: 28_205_601,
    puid: "wikidata/28205601",
    name: "RIPscrip version 1 Icon",
    extensions: &["icn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
