use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_45347570: FileFormat = FileFormat {
    id: 45_347_570,
    puid: "wikidata/45347570",
    name: "Lotus 1-2-3 Worksheet file format, version 4-5",
    extensions: &["wk4", "wk4"],
    media_types: &["application/lotus123", "application/vnd.lotus-1-2-3"],
    internal_signatures: &[],
    related_formats: &[],
};
