use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120966179: FileFormat = FileFormat {
    id: 120_966_179,
    puid: "wikidata/120966179",
    name: "Microsoft Money 98 data file",
    extensions: &["mn6"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
