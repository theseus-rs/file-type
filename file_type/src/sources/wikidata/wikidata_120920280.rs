use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120920280: FileFormat = FileFormat {
    id: 120_920_280,
    puid: "wikidata/120920280",
    name: "Microsoft Money 2002 data",
    extensions: &["m10"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
