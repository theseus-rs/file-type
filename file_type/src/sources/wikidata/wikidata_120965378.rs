use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_120965378: FileFormat = FileFormat {
    id: 120_965_378,
    puid: "wikidata/120965378",
    name: "Microsoft Money version 2 data",
    extensions: &["mn2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
