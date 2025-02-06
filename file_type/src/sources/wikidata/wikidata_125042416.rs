use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125042416: FileFormat = FileFormat {
    id: 125_042_416,
    source_type: SourceType::Wikidata,
    name: "MIDI-Learn file",
    extensions: &["xly"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
