use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855536: FileFormat = FileFormat {
    id: 105_855_536,
    source_type: SourceType::Wikidata,
    name: "OpenColorIO profile (with rem)",
    extensions: &["ocio"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
