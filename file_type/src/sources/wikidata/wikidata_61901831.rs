use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61901831: FileFormat = FileFormat {
    id: 61_901_831,
    puid: "wikidata/61901831",
    name: "Peak Graphical Waveform File",
    extensions: &["pk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
