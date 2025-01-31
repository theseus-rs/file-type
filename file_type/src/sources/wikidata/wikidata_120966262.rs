use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120966262: FileFormat = FileFormat {
    id: 120_966_262,
    puid: "wikidata/120966262",
    name: "Microsoft Money 99 data",
    extensions: &["mn7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
