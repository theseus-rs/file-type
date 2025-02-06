use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123298791: FileFormat = FileFormat {
    id: 123_298_791,
    source_type: SourceType::Wikidata,
    name: "Retrospect RXT File",
    extensions: &["rxt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
