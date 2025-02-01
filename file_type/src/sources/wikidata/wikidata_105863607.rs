use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105863607: FileFormat = FileFormat {
    id: 105_863_607,
    puid: "wikidata/105863607",
    name: "Metalink (v4)",
    extensions: &["meta4"],
    media_types: &["application/metalink4+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
