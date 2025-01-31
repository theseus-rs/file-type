use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_6457314: FileFormat = FileFormat {
    id: 6_457_314,
    puid: "wikidata/6457314",
    name: "LBR",
    extensions: &["lbr", "lqr", "lyr", "lzr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
