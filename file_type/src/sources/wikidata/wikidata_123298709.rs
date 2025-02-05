use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123298709: FileFormat = FileFormat {
    id: 123_298_709,
    source_type: SourceType::Wikidata,
    name: "Retrospect RBC File",
    extensions: &["rbc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
