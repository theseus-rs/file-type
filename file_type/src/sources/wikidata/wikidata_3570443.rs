use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3570443: FileFormat = FileFormat {
    id: 3_570_443,
    puid: "wikidata/3570443",
    name: "Xtremsplit file format",
    extensions: &["xtm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
