use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_125042416: FileFormat = FileFormat {
    id: 125_042_416,
    puid: "wikidata/125042416",
    name: "MIDI-Learn file",
    extensions: &["xly"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
