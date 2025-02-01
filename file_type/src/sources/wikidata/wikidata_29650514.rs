use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29650514: FileFormat = FileFormat {
    id: 29_650_514,
    puid: "wikidata/29650514",
    name: "packPNM",
    extensions: &["ppn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
