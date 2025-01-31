use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205604: FileFormat = FileFormat {
    id: 28_205_604,
    puid: "wikidata/28205604",
    name: "RIPscrip version 1 Icon Mask",
    extensions: &["msk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
