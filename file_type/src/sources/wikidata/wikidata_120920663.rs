use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120920663: FileFormat = FileFormat {
    id: 120_920_663,
    puid: "wikidata/120920663",
    name: "Microsoft Money 2003 data",
    extensions: &["m11"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
