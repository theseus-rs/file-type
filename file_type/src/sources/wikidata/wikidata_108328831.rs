use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_108328831: FileFormat = FileFormat {
    id: 108_328_831,
    puid: "wikidata/108328831",
    name: "Universe Sandbox Data File",
    extensions: &["ubox"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
