use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120920758: FileFormat = FileFormat {
    id: 120_920_758,
    puid: "wikidata/120920758",
    name: "Microsoft Money 2005 backup data",
    extensions: &["m14"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
