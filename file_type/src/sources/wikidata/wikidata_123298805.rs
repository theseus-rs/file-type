use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123298805: FileFormat = FileFormat {
    id: 123_298_805,
    source_type: SourceType::Wikidata,
    name: "Retrospect RDX File",
    extensions: &["rdx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
