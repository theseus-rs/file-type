use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28975863: FileFormat = FileFormat {
    id: 28_975_863,
    puid: "wikidata/28975863",
    name: "OOGL Bezier Surface BEZ",
    extensions: &["bez"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
