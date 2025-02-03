use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123298805: FileFormat = FileFormat {
    id: 123_298_805,
    source_type: SourceType::Wikidata,
    name: "Retrospect RDX File",
    extensions: &["rdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
