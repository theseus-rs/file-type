use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205614: FileFormat = FileFormat {
    id: 28_205_614,
    puid: "wikidata/28205614",
    name: "RIPscrip version 2 Icon Mask",
    extensions: &["bmm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
