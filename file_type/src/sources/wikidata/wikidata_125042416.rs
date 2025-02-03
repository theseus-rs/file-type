use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_125042416: FileFormat = FileFormat {
    id: 125_042_416,
    source_type: SourceType::Wikidata,
    name: "MIDI-Learn file",
    extensions: &["xly"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
