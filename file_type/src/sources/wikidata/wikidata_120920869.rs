use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120920869: FileFormat = FileFormat {
    id: 120_920_869,
    puid: "wikidata/120920869",
    name: "Microsoft Money 2007 data file",
    extensions: &["m16"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
