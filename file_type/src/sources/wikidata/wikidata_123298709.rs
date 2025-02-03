use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123298709: FileFormat = FileFormat {
    id: 123_298_709,
    source_type: SourceType::Wikidata,
    name: "Retrospect RBC File",
    extensions: &["rbc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
