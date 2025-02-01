use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120965738: FileFormat = FileFormat {
    id: 120_965_738,
    puid: "wikidata/120965738",
    name: "Microsoft Money 95 data file",
    extensions: &["mn4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
